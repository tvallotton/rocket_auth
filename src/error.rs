use crate::prelude::*;
use std::error::Error as ErrorTrait;
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorKind {
    MutexPoisonError,
    SystemTimeError(Duration),
    SQLiteError,
    NoneError,
    Argon2ParsingError,
    ClientSessionError,
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

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
    // #[serde(skip_serializing)]
    
    
}

impl ErrorTrait for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

pub fn raise<T>(kind: ErrorKind, msg: &str) -> Result<T> {
    Err(Error {
        message: msg.into(),
        kind,
    })
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
        Error {
            message: format!("{}", error),
            kind: ErrorKind::MutexPoisonError,
        }
    }
}

#[cfg(feature = "sqlite-db")]
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::SQLiteError,
        }
    }
}

use std::time::SystemTimeError;
impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::SystemTimeError(error.duration()),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::Argon2ParsingError,
        }
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Error {
        Error {
            message: "".into(),
            kind: ErrorKind::Unspecified,
        }
    }
}

impl From<&Error> for Error {
    fn from(error: &Error) -> Error {
        error.clone()
    }
}

impl From<redis::RedisError> for Error {
    fn from(error: redis::RedisError) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::RedisError,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::JsonParsingError,
        }
    }
}

impl From<tokio_postgres::Error> for Error {

    fn from(error: tokio_postgres::Error) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::PostgresqlError,
        }
    }
}

impl From<std::io::Error> for Error {

    fn from(error: std::io::Error) -> Error {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::IOError,
        }
    }
}