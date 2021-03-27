pub use crate::db::DBConnection;
pub use crate::error::{Error, raise, ErrorKind, SetErrorMessage};
pub use crate::cookies::Session;
pub use crate::session::SessionManager;
pub use crate::{User, Users, Login, Signup};

pub use std::time::Duration;
pub use crate::session::AuthKey;
pub use std::error::Error as StdError;
pub use serde::{Deserialize, Serialize};
pub use std::ops::Deref;
pub type Result<T, E = Error> = std::result::Result<T, E>;