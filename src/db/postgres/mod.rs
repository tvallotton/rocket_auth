use crate::prelude::{Result, *};
mod sql;
use sql::*;

use sqlx::postgres::PgPool;

use sqlx::*;

#[rocket::async_trait]
impl DBConnection for PgPool {
    async fn init(&self) -> Result<()> {
        query(CREATE_TABLE).execute(self).await?;
        Ok(())
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        query(INSERT_USER)
            .bind(email)
            .bind(hash)
            .bind(is_admin)
            .execute(self)
            .await?;
        Ok(())
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        query(UPDATE_USER)
            .bind(user.id)
            .bind(&user.email)
            .bind(&user.password)
            .bind(user.is_admin)
            .execute(self)
            .await?;

        Ok(())
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        query(REMOVE_BY_ID).bind(user_id).execute(self).await?;
        Ok(())
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        query(REMOVE_BY_EMAIL).bind(email).execute(self).await?;
        Ok(())
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        let user = query_as(SELECT_BY_ID).bind(user_id).fetch_one(self).await?;

        Ok(user)
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let user = query_as(SELECT_BY_EMAIL)
            .bind(email)
            .fetch_one(self)
            .await?;
        Ok(user)
    }
}
