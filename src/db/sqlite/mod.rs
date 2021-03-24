mod sql;

use crate::prelude::*;

use rusqlite::{params, Connection};
use sql::*;
use std::sync::Mutex;

impl DBConnection for Mutex<Connection> {
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<()> {
        let db = self.lock()?;
        db.execute(INSERT_USER, params![email, hash, is_admin])?;
        Ok(())
    }
    fn update_user(&self, user: User) -> Result<()> {
        todo!()
    }
    fn delete_user(&self, user_id: u32) -> Result<()> {
        todo!()
    }
    fn get_user_by_id(&self, user_id: u32) -> Result<User> {
        todo!()
    }
    fn get_user_by_email(&self, email: &str) -> Result<User> {
        todo!()
    }
}
