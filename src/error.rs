use rocket::Responder;
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
    SQLiteError(#[from] rusqlite::Error),

    #[error("Argon2ParsingError: {0}")]
    Argon2ParsingError(#[from] argon2::Error),

    #[error("UnaothorizedError")]
    UnaothorizedError,

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
    #[error("InvalidCredentialsError: Incorrect email or password.")]
    InvalidCredentialsError,
    #[error("UnsafePasswordTooShort")]
    UnsafePasswordTooShort,
    #[error("UnauthorizedError")]
    UnauthorizedError,
    #[cfg(feature = "redis-session")]
    #[error("RedisError")]
    RedisError(#[from] redis::Error),
    #[error("SerdeError: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[cfg(feature = "postgres-db")]
    #[error("PostgresqlError: {0}")]
    PostgresqlError(#[from] tokio_postgres::Error),
    #[error("IOError: {0}")]
    IOError(#[from] std::io::Error),
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
