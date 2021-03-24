

#[cfg(feature = "sqlite-db")]
mod sqlite;
// mod postgres;

use crate::prelude::*;

pub trait DBConnection {
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
    fn update_user(&self, user: User) -> Result<()>;
    fn delete_user(&self, user_id: u32) -> Result<()>;
    fn get_user_by_id(&self, user_id: u32) -> Result<User>;
    fn get_user_by_email(&self, email: &str) -> Result<User>;
}
