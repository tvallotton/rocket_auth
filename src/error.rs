use std::*;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("That is not a valid email address.")]
    InvalidEmailAddressError,

    #[error("That email address already exists. Try logging in.")]
    EmailAlreadyExists,

    #[cfg(feature = "sqlx-sqlite")]
    #[error("The mutex guarding the Sqlite connection was posioned.")]
    MutexPoisonError,

    #[error("An error occured trying to retrieve the current time.")]
    SystemTimeError(#[from] time::SystemTimeError),

    #[error("Could not find any user that fits the specified requirements.")]
    UserNotFoundError,

    #[cfg(feature = "sqlx-sqlite")]
    #[cfg(feature = "sqlx-postgres")]
    #[error("SqlxError: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Argon2ParsingError: {0}")]
    Argon2ParsingError(#[from] argon2::Error),

    #[error("Unspecified")]
    Unspecified,

    #[cfg(feature = "rusqlite")]
    #[error("RusqliteError: {0}")]
    RusqliteError(#[from] rusqlite::Error),

    #[error("UserIsNotAdmin: the queried user was not an administrator.")]
    UserIsNotAdmin,

    #[error("QueryError")]
    QueryError,

    #[error("UnmanagedStateError")]
    UnmanagedStateError,

    #[error("FormValidationError")]
    FormValidationError,

    #[error("UnauthenticatedError: The operation failed because the client is not authenticated.")]
    UnauthenticatedError,

    #[error("The email \"{0}\" is not registered. Try signing up first.")]
    EmailDoesNotExist(String),

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
    #[cfg(feature = "redis")]
    #[error("RedisError")]
    RedisError(#[from] redis::RedisError),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[cfg(feature = "sqlx-postgres")]
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
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
