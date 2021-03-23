
use std::error::Error as ErrorTrait;
use std::
    fmt::{self, Display, Formatter};
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub source: Option<Box<dyn ErrorTrait>>,
}
impl ErrorTrait for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Original error: {:?}\nError message: {}",
            self.source, self.message
        )
    }
}

pub fn raise<T>(msg: &str) -> Result<T> {
    Err(Error {
        message: msg.into(),
        source: None,
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

impl<T> SetErrorMessage for Option<T> {
    type Ok = T;
    fn msg(self, msg: &str) -> std::result::Result<T, Error> {
        match self {
            Some(val) => Ok(val),
            None => {
                let error = Error {
                    message: msg.into(),
                    source: None,
                };
                Err(error)
            }
        }
    }
}

/*****  CONVERSIONS  *****/

// impl<E: ErrorTrait> From<E> for Error {
//     fn from(error: E) -> Error {
//         Error {
//             message: "PoisonError: Mutex was poisoned".into(),
//             source: Some(Box::new(error))
//         }
//     }
// }

use std::sync::PoisonError;
impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error {
            message: "PoisonError: Mutex was poisoned".into(),
            source: None,
        }
    }
}

#[cfg(feature = "sqlite")]
impl From<rusqlite::Error> for Error {
    fn from(error: rusqlite::Error) -> Error {
        Error {
            message: "SQLiteError".into(),
            source: Some(Box::new(error)),
        }
    }
}

use std::time::SystemTimeError;
impl From<SystemTimeError> for Error {
    fn from(error: SystemTimeError) -> Error {
        Error {
            message: "SystemTimeError".into(),
            source: Some(Box::new(error)),
        }
    }
}
use std::convert::Infallible;
impl From<Infallible> for Error {
    fn from(error: Infallible) -> Error {
        Error {
            message: "Infallible".into(),
            source: Some(Box::new(error)),
        }
    }
}

impl From<argon2::Error> for Error {
    fn from(error: argon2::Error) -> Error {
        Error {
            message: "Argon2Error".into(),
            source: Some(Box::new(error)),
        }
    }
}

// type RocketError = (rocket::http::Status, std::result::Result<(), Error>);
// impl From<RocketError> for Result<(), (rocket::http::Status, Error)> {

// }
// use rocket::request::FormParseError;
// impl<'f> From<FormParseError<'f>> for Error {
//     fn from(error: FormParseError) -> Error {
//         Error {
//             message: format!("{:?}", error),
//             source: None,
//         }
//     }
// }
