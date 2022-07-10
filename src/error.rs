use self::Error::*;
use crate::language::messages;
use crate::Config;
use rocket::http::{self, ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_lang::LangCode;
use serde_json::{json, to_string};
use std::convert::TryFrom;
use std::io::Cursor;
use thiserror::Error;
use ValidationError::IncorrectPassword;

/// The Error enum represents every possible error that `rocket_aut` may return.
/// It implements [`rocket::response::Responder`](Responder), so it can be ealisly used
/// in API endpoints that are expected to return a json response. The structure for the
/// json response is the following:
/// ```json
/// {
///     "status": "error",
///     "code": 400
///     "type": "invalid_credentials",
///     "messages": ["Incorrect email or password."]
/// }
/// ```
/// The code field contains the HTTP status code, and the "messages" field contains a list of
/// error messages.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// This error occurs when a user is trying to access a resource that
    /// requires authentication, and they are not logged in.
    #[error("unauthorized: authentication is needed to access this resource")]
    Unauthorized,

    /// This error is thrown when attempting to access a resource available for admins only.
    /// The http status code of this response is Forbidden 403.
    #[error("forbidden: you don't have permission to access this resource")]
    Forbidden,

    /// This error is thrown when the user input for a request isn't valid.
    /// The http status code for this response can be either BadRequest 400 or Unauthorized 401.
    #[error("user input validation failed")]
    Validation(Vec<ValidationError>),

    /// This error can be thrown for multiple different reasons.
    /// The http status code for this response is InternalServerError 500.
    #[error("internal server error")]
    Server(#[source] InternalServerError),

    #[error("attempted to perform a stateful action through a safe http method: expected an unsafe http method, got {0}", )]
    HttpMethod(http::Method),
}

/// The vaidation error
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ValidationError {
    /// the email does not belong to a registered user
    #[error("the email {0:?} does not belong to a registered user")]
    UserNotFound(String),

    /// the email address is not valid
    #[error("the email address is not valid")]
    InvalidEmailAddress,

    /// that email already exists, try logging in
    #[error("that email {0:?} already exists, try logging in")]
    EmailAlreadyExists(String),

    /// the password should be at least 8 characters long
    #[error("the password should be at least 8 characters long")]
    PasswordTooShort,

    /// the password should have at least one upper case letter
    #[error("the password should have at least one upper case letter")]
    PasswordMissingUppercase,
    /// the password should have at least one lowercase letter
    #[error("the password should have at least one lowercase letter")]
    PasswordMissingLowercase,

    /// the password should have at least one number
    #[error("the password should have at least one number")]
    PasswordMissingNumber,

    /// incorrect email or password
    #[error("incorrect email or password")]
    IncorrectPassword,
}

#[derive(Error, Debug)]
pub enum InternalServerError {
    /// This error is thrown when trying to retrieve `Users` but it isn't being managed by the app.
    /// It can be fixed adding `.manage(users)` to the app, where `users` is of type `Users`.
    #[error("failed to retrieve `Users`. You may be missing `.manage(users)` in your app")]
    UnmanagedStateError, // used

    /// A wrapper around [`sqlx::Error`].
    #[cfg(any(feature = "sqlx"))]
    #[error("sqlx failure")]
    SQLx(
        #[source]
        #[from]
        sqlx::Error,
    ),

    /// A wrapper around [`argon2::Error`].
    #[error("failed to validate password")]
    Argon2(
        #[source]
        #[from]
        argon2::Error,
    ),

    /// A wrapper around [`rusqlite::Error`].
    #[cfg(feature = "rusqlite")]
    #[error("rusqlite failure")]
    Rusqlite(
        #[source]
        #[from]
        rusqlite::Error,
    ),

    /// A wrapper around [`redis::RedisError`].
    #[cfg(feature = "redis")]
    #[error("redis failure")]
    Redis(
        #[source]
        #[from]
        redis::RedisError,
    ),

    /// A wrapper around [`serde_json::Error`].
    #[error("serde failure")]
    Serde(
        #[source]
        #[from]
        serde_json::Error,
    ),

    /// A wrapper around [`std::io::Error`].
    #[cfg(feature = "sqlx-postgres")]
    #[error("io failure")]
    IO(
        #[source]
        #[from]
        std::io::Error,
    ),

    /// A wrapper around [`tokio_postgres::Error`].
    #[cfg(feature = "tokio-postgres")]
    #[error("tokio postgres failure")]
    TokioPostgres(
        #[source]
        #[from]
        tokio_postgres::Error,
    ),
}

/*****  CONVERSIONS  *****/
impl From<Vec<ValidationError>> for Error {
    fn from(error: Vec<ValidationError>) -> Error {
        Error::Validation(error)
    }
}
impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Error {
        Error::Validation(vec![error])
    }
}
impl<E> From<E> for Error
where
    E: Into<InternalServerError>,
{
    fn from(error: E) -> Self {
        Error::Server(error.into())
    }
}

impl Error {
    #[allow(dead_code)]
    fn log(&self) {
        use std::fmt::Write;
        let mut msg = format!("{self}");
        let mut dyn_err: &dyn std::error::Error = self;
        while let Some(src) = dyn_err.source() {
            write!(msg, ": {src}").ok();
            dyn_err = src;
        }
        write!(msg, ".").ok();
        log::error!("{}", msg.to_lowercase());
    }

    fn status(&self) -> Status {
        match self {
            Unauthorized => Status::Unauthorized,
            Forbidden => Status::Forbidden,
            Validation(error) if matches!(error[0], IncorrectPassword) => Status::Unauthorized,
            Validation(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        }
    }
}

fn default_responder<'r>(err: Error, req: &'r Request<'_>) -> response::Result<'static> {
    let lang = LangCode::try_from(req).unwrap_or(LangCode::En);
    let messages = messages(&err, lang);
    let payload = to_string(&json!({
        "status": "error",
        "code": err.status().code,
        "message": messages,
    }))
    .unwrap();

    Response::build()
        .sized_body(payload.len(), Cursor::new(payload))
        .header(ContentType::new("application", "json"))
        .ok()
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let config = Config::from_request(req);
        if let Some(f) = config.error_response {
            f(self, req)
        } else {
            default_responder(self, req)
        }
    }
}
