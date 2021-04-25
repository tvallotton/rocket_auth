use crate::prelude::*;
use std::error::Error as ErrorTrait;
use std::fmt::{self, Display, Formatter};






#[derive(Debug, Clone)]
pub enum Error {
    #[cfg(feature = "sqlite-db")]
    MutexPoisonError,
    SystemTimeError,
    #[cfg(feature = "sqlite-db")]
    SQLiteError(rusqlite::Error),
    NoneError,
    Argon2ParsingError,
    AunothorizedError,
    Unspecified,
    QueryError,
    UnmanagedStateError,
    FormValidationError,
    UnauthenticatedClientError,
    UnsafePasswordError,
    Unauthorized,
    RedisError,
    JsonParsingError,
    PostgresqlError,
    IOError
}


impl ErrorTrait for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}


/*****  MESSAGE PASSING  *****/
pub trait SetErrorMessage {
    type Ok;
    fn msg(self, msg: &str) -> std::result::Result<Self::Ok, Error>;
}

impl<T, E: Into<Error> + ErrorTrait> SetErrorMessage for std::result::Result<T, E> {
    type Ok = T;
    fn msg(self, msg: &str) -> std::result::Result<T, Error> {
        match self {
            Ok(val) => Ok(val),
            Err(error) => {
                let mut error: Error = error.into();
                error.message = msg.into();
                Err(error)
            }
        }
    }
}

/*****  CONVERSIONS  *****/
use std::sync::PoisonError;
impl<T> From<PoisonError<T>> for Error {
    fn from(error: PoisonError<T>) -> Error {
        Error::MutexPoisonError
    }
}

#[cfg(feature = "sqlite-db")]
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        Error::SQLiteError(error)
        
    }
}

use std::time::SystemTimeError;
impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Error {
        Error::SystemTimeError(error)
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Error {
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
        error.clone()
    }
}
#[cfg(feature="redis-session")]
impl From<redis::RedisError> for Error {
    fn from(error: redis::RedisError) -> Error {
        Error::RedisError
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::JsonParsingError

    }
}
#[cfg(feature="postgres-db")]
impl From<tokio_postgres::Error> for Error {

    fn from(error: tokio_postgres::Error) -> Error {
        Error::PostgresqlError
    }
}

impl From<std::io::Error> for Error {

    fn from(error: std::io::Error) -> Error {
        Error::IOError
    }
}