


use crate::prelude::*;

use rusqlite::Connection;
use std::sync::Mutex;


impl DBConnection for Mutex<Connection> {
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<u32> {
        todo!()
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
