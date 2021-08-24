// pub use crate::language::Language;
pub use crate::cookies::Session;
pub use crate::error::Error;
pub use crate::session::SessionManager;
pub use crate::{AdminUser, Auth, Login, Signup, User, Users};

pub(crate) use crate::db::DBConnection;
pub(crate) use async_trait::async_trait;
pub(crate) use fehler::*;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::ops::Deref;
pub(crate) use std::time::Duration;

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub fn now() -> i64 {
    chrono::Utc::now().timestamp()
}
