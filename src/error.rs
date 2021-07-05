use std::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("That is not a valid email address.")]
    InvalidEmailAddressError,

    #[error("That email address already exists. Try logging in.")]
    EmailAlreadyExists,

    #[cfg(feature = "sqlite-db")]
    #[error("The mutex guarding the Sqlite connection was posioned.")]
    MutexPoisonError,

    #[error("An error occured trying to retrieve the current time.")]
    SystemTimeError(#[from] time::SystemTimeError),

    #[error("Could not find any user that fits the specified requirements.")]
    UserNotFoundError,

    #[cfg(feature = "sqlite-db")]
    #[error("RusqliteError: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2ParsingError: {0}")]
    Argon2ParsingError(#[from] argon2::Error),

    #[error("Unspecified")]
    Unspecified,

    #[error("Unspecified")]
    QueryError,

    #[error("UnmanagedStateError")]
    UnmanagedStateError,

    #[error("FormValidationError")]
    FormValidationError,

    #[error("UnauthenticatedError: The operation failed because the client is not authenticated.")]
    UnauthenticatedError,

    #[error("Incorrect email or password.")]
    InvalidCredentialsError,
    #[error("The password must be at least 8 characters long.")]
    UnsafePasswordTooShort,

    #[error("The password must include a digit.")]
    UnsafePasswordHasNoDigit,

    #[error("The password must include an upper case character.")]
    UnsafePasswordHasNoUpper,

    #[error("The password must include a lower case character.")]
    UnsafePasswordHasNoLower,

    #[error("Incorrect email or password")]
    UnauthorizedError,
    #[cfg(feature = "redis-session")]
    #[error("RedisError")]
    RedisError(#[from] redis::RedisError),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[cfg(feature = "postgres-db")]
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "tokio-postgres-db")]
    #[error("TokioPostgresError: {0}")]
    TokioPostgresError(#[from] tokio_postgres::Error),
}

/*****  CONVERSIONS  *****/
#[cfg(feature = "sqlite-db")]
use std::sync::PoisonError;
#[cfg(feature = "sqlite-db")]
impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Error {
        Error::MutexPoisonError
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Error {
        Error::Unspecified
    }
}
use self::Error::*;
impl Error {
    fn message(&self) -> String {
        match self {
            InvalidEmailAddressError
            | InvalidCredentialsError
            | EmailAlreadyExists
            | UnauthorizedError
            | UserNotFoundError
            | UnsafePasswordTooShort
            | UnsafePasswordHasNoDigit
            | UnsafePasswordHasNoLower
            | UnsafePasswordHasNoUpper => format!("{}", self),
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

