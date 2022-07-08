pub use crate::forms::ValidationError;
use std::*;


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
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error is thrown when trying to retrieve `Users` but it isn't being managed by the app.
    /// It can be fixed adding `.manage(users)` to the app, where `users` is of type `Users`.
    #[error("UnmanagedStateError: failed retrieving `Users`. You may be missing `.manage(users)` in your app.")]
    UnmanagedStateError, // used

    /// This error occurs when a user is trying to access a resource that 
    /// requires authentication, and they are not logged in. 
    #[error("Authentication is needed to access this resource.")]
    Unauthorized, 

    /// This error is thrown when attempting to access a resource available for admins only. 
    #[error("Forbidden: you don't have permission to access this resource.")]
    Forbidden,

    #[error("{0:?}")]
    Validation(Vec<crate::forms::ValidationError>),

    /// A wrapper around [`sqlx::Error`].
    #[cfg(any(feature = "sqlx"))]
    #[error("SQLxError: {0}")]
    SqlxError(#[from] sqlx::Error),

    /// A wrapper around [`argon2::Error`].
    #[error("Argon2ParsingError: {0}")]
    Argon2ParsingError(#[from] argon2::Error),

    /// A wrapper around [`rusqlite::Error`].
    #[cfg(feature = "rusqlite")]
    #[error("RusqliteError: {0}")]
    RusqliteError(#[from] rusqlite::Error),

    /// A wrapper around [`redis::RedisError`].
    #[cfg(feature = "redis")]
    #[error("RedisError")]
    RedisError(#[from] redis::RedisError),

    /// A wrapper around [`serde_json::Error`].
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),

    /// A wrapper around [`std::io::Error`].
    #[cfg(feature = "sqlx-postgres")]
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),

    /// A wrapper around [`tokio_postgres::Error`].
    #[cfg(feature = "tokio-postgres")]
    #[error("TokioPostgresError: {0}")]
    TokioPostgresError(#[from] tokio_postgres::Error),
}

use std::convert::TryFrom;
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

///```
/// {
///     "status": "400",
///     "error": ["internal server "]
/// }
///
/// ```

impl Error {
    fn status(&self) -> Status {
        match self {
            Error::Unauthorized => Status::Unauthorized,
            Error::Forbidden => Status::Forbidden,
            Error::Validation(_) => Status::BadRequest,
            _ => Status::InternalServerError,
        }
    }
    fn message(&self, lang: LangCode) -> Value {
        match self {
            Error::Validation(error) => {
                json!("")
            }
            _ => {
                json!("internal server error")
            }
        }
    }
}

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

use rocket_lang::LangCode;
use serde_json::*;
use std::io::Cursor;

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let lang = LangCode::try_from(req).unwrap_or(LangCode::En);

        let payload = to_string(&json!({
            "status": self.status().code,
            "message": self.message(lang),
        }))
        .unwrap();

        Response::build()
            .sized_body(payload.len(), Cursor::new(payload))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}
