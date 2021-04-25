use std::error::Error as ErrorTrait;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    InvalidEmailAddressError,
    EmailAlreadyExists,
    #[cfg(feature = "sqlite-db")]
    MutexPoisonError,
    SystemTimeError,
    UserNotFoundError,
    #[cfg(feature = "sqlite-db")]
    SQLiteError,
    NoneError,
    Argon2ParsingError,
    AunothorizedError,
    Unspecified,
    QueryError,
    UnmanagedStateError,
    FormValidationError,
    UnauthenticatedClientError,
    InvalidCredentialsError,
    UnsafePasswordTooShort,
    UnauthorizedError,
    RedisError,
    JsonParsingError,
    PostgresqlError,
    IOError,
}

impl ErrorTrait for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}




/*****  CONVERSIONS  *****/
use std::sync::PoisonError;
impl<T> From<PoisonError<T>> for Error {
    fn from(_error: PoisonError<T>) -> Error {
        Error::MutexPoisonError
    }
}


#[cfg(feature = "sqlite-db")]
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        use rusqlite::Error::*;
        match error {
            SqliteFailure(_, Some(message)) => {
                if message == "UNIQUE constraint failed: users.email" {
                    Error::EmailAlreadyExists
                } else {
                    Error::SQLiteError
                }
            },
            _ => Error::SQLiteError
        }

    }
}

use std::time::SystemTimeError;
impl From<SystemTimeError> for Error {
    fn from(_error: SystemTimeError) -> Error {
        Error::SystemTimeError
    }
}

impl From<argon2::Error> for Error {
    fn from(_error: argon2::Error) -> Error {
        Error::Argon2ParsingError
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Error {
        Error::Unspecified
    }
}

impl From<&Error> for Error {
    fn from(error: &Error) -> Error {
        *error
    }
}
#[cfg(feature = "redis-session")]
impl From<redis::RedisError> for Error {
    fn from(_error: redis::RedisError) -> Error {
        Error::RedisError
    }
}

impl From<serde_json::Error> for Error {
    fn from(_error: serde_json::Error) -> Error {
        Error::JsonParsingError
    }
}
#[cfg(feature = "postgres-db")]
impl From<tokio_postgres::Error> for Error {
    fn from(_error: tokio_postgres::Error) -> Error {
        Error::PostgresqlError
    }
}

impl From<std::io::Error> for Error {
    fn from(_error: std::io::Error) -> Error {
        Error::IOError
    }
}
