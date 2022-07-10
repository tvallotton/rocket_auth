

pub use crate::error::Error;
pub use crate::error::ValidationError; 
pub use crate::error::InternalServerError; 
pub use crate::forms::{Login, Signup};
pub use crate::{AdminUser, Auth, User, Users};
/// A type alias of result to omit the error type.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub(crate) use crate::cookies::Session;
pub(crate) use crate::db::DBConnection;
pub(crate) use crate::session::SessionManager;
pub(crate) use async_trait::async_trait;
pub(crate) use fehler::*;
pub(crate) use rocket::form::FromForm;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::fmt::Debug;
pub(crate) use std::ops::Deref;
pub(crate) use std::time::Duration;
pub(crate) fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

macro_rules! try_outcome {
    ($outcome: expr) => {
        match $outcome {
            rocket::outcome::Outcome::Success(success) => success,
            rocket::outcome::Outcome::Failure(failure) => {
                return rocket::outcome::Outcome::Failure(failure)
            }
            rocket::outcome::Outcome::Forward(forward) => {
                return rocket::outcome::Outcome::Forward(forward)
            }
        }
    };
    ($outcome: expr, err: $err:expr) => {
        match $outcome {
            rocket::outcome::Outcome::Success(success) => success,
            rocket::outcome::Outcome::Failure(failure) => {
                return rocket::outcome::Outcome::Failure((failure.0, $err.into()))
            }
            rocket::outcome::Outcome::Forward(forward) => {
                return rocket::outcome::Outcome::Forward(forward)
            }
        }
    };
}
pub(crate) use try_outcome;
