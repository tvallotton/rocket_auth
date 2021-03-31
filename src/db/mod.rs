#[cfg(feature = "postgres-db")]
mod postgres;

#[cfg(feature = "sqlite-db")]
mod sqlite;


use crate::prelude::*;

pub trait DBConnection: Send + Sync {
    fn init(&self) -> Result<()>;
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
    fn update_user(&self, user: &User) -> Result<()>;
    fn delete_user_by_id(&self, user_id: i32) -> Result<()>;
    fn delete_user_by_email(&self, email: &str) -> Result<()>;
    fn get_user_by_id(&self, user_id: i32) -> Result<User>;
    fn get_user_by_email(&self, email: &str) -> Result<User>;
}
