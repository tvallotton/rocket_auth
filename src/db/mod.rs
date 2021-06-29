#[cfg(feature = "postgres-db")]
mod postgres;

#[cfg(feature = "sqlite-db")]
mod sqlite;

#[cfg(feature = "tokio-postgres-db")]
mod tokio_postgres;

use crate::prelude::*;

#[rocket::async_trait]
pub trait DBConnection: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()>;
    async fn delete_user_by_email(&self, email: &str) -> Result<()>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<User>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
}
