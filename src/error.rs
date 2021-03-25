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
    QueryError
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}



impl ErrorTrait for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

pub fn raise<T>(msg: &str) -> Result<T> {
    Err(Error {
        message: msg.into(),
        kind: ErrorKind::Unspecified,
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
    fn from(_: PoisonError<T>) -> Error {
        Error {
            message: "".into(),
            kind: ErrorKind::MutexPoisonError,
        }
    }
}

#[cfg(feature = "sqlite-db")]
impl From<rusqlite::Error> for Error {
    fn from(_: rusqlite::Error) -> Error {
        Error {
            message: "".into(),
            kind: ErrorKind::SQLiteError,
        }
    }
}

use std::time::SystemTimeError;
impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Error {
        Error {
            message: "".into(),
            kind: ErrorKind::SystemTimeError(error.duration()),
        }
    }
}


impl From<argon2::Error> for Error {
    fn from(_: argon2::Error) -> Error {
        Error {
            message: "".into(),
            kind: ErrorKind::Argon2ParsingError,
        }
    }
}
