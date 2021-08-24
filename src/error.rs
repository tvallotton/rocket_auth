use std::*;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error occurs when attempting to create a user with an invalid email address.
    #[error("That is not a valid email address.")]
    InvalidEmailAddressError,

    /// This error only occurs if the application panics while holding a locked mutex.
    #[cfg(feature = "sqlx-sqlite")]
    #[error("The mutex guarding the Sqlite connection was posioned.")]
    MutexPoisonError,

    /// Thrown when the requested user does not exists.
    #[error("Could not find any user that fits the specified requirements.")]
    UserNotFoundError,

    /// This error is thrown when trying to retrieve `Users` but it isn't being managed by the app.
    /// It can be fixed adding `.manage(users)` to the app, where `users` is of type `Users`.
    #[error("UnmanagedStateError: failed retrieving `Users`. You may be missing `.manage(users)` in your app.")]
    UnmanagedStateError,

    #[error("UnauthenticatedError: The operation failed because the client is not authenticated.")]
    UnauthenticatedError,
    /// This error occurs when a user tries to log in, but their account doesn't exists.
    #[error("The email \"{0}\" is not registered. Try signing up first.")]
    EmailDoesNotExist(String),
    /// This error is thrown when a user tries to sign up with an email that already exists.
    #[error("That email address already exists. Try logging in.")]
    EmailAlreadyExists,
    /// This error occurs when the user does exists, but their password was incorrect.
    #[error("Incorrect email or password")]
    UnauthorizedError,

    /// A wrapper around [`validator::ValidationError`].
    #[error("{0}")]
    FormValidationError(#[from] validator::ValidationError),

    /// A wrapper around [`validator::ValidationErrors`].
    #[error("FormValidationErrors: {0}")]
    FormValidationErrors(#[from] validator::ValidationErrors),

    /// A wrapper around [`sqlx::Error`].
    #[cfg(any(feature = "sqlx-sqlite", feature = "sqlx-postgres"))]
    #[error("SqlxError: {0}")]
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

/*****  CONVERSIONS  *****/
#[cfg(feature = "sqlx-sqlite")]
use std::sync::PoisonError;
#[cfg(feature = "sqlx-sqlite")]
impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Error {
        Error::MutexPoisonError
    }
}

use self::Error::*;
impl Error {
    fn message(&self) -> String {
        match self {
            InvalidEmailAddressError
            | EmailAlreadyExists
            | UnauthorizedError
            | UserNotFoundError => format!("{}", self),
            FormValidationErrors(source) => {
                source
                    .field_errors()
                    .into_iter()
                    .map(|(_, error)| error)
                    .map(IntoIterator::into_iter)
                    .map(|errs| {
                        errs //
                            .map(|err| &err.code)
                            .fold(String::new(), |a, b| a + b)
                    })
                    .fold(String::new(), |a, b| a + &b)
            }
            #[cfg(debug_assertions)]
            e => return format!("{}", e),
            #[allow(unreachable_patterns)]
            _ => "undefined".into(),
        }
    }
}

use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_json::*;
use std::io::Cursor;

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let payload = to_string(&json!({
            "status": "error",
            "message": self.message(),
        }))
        .unwrap();
        Response::build()
            .sized_body(payload.len(), Cursor::new(payload))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}
