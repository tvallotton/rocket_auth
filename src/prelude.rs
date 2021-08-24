// pub use crate::language::Language;
pub use crate::cookies::Session;
pub use crate::error::Error;
pub use crate::forms::{Login, Signup};

pub use crate::{AdminUser, Auth, User, Users};
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) use crate::session::SessionManager;
pub(crate) use crate::db::DBConnection;
pub(crate) use async_trait::async_trait;
pub(crate) use fehler::*;
pub(crate) use rocket::form::FromForm;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::fmt::Debug;
pub(crate) use std::ops::Deref;
pub(crate) use std::time::Duration;
pub(crate) use validator::{Validate, ValidationError};
pub(crate) fn now() -> i64 {
    chrono::Utc::now().timestamp()
}
