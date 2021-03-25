use crate::prelude::*;
use tokio_postgres::Client;

impl DBConnection for Client {
    fn init(&self) -> Result<()> {
        todo!()
    }
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        todo!()
    }
    fn update_user(&self, user: User) -> Result<()> {
        todo!()
    }
    fn delete_user_by_id(&self, user_id: u32) -> Result<()> {
        todo!()
    }
    fn delete_user_by_email(&self, email: &str) -> Result<()> {
        todo!()
    }
    fn get_user_by_id(&self, user_id: u32) -> Result<User> {
        todo!()
    }
    fn get_user_by_email(&self, email: &str) -> Result<User> {
        todo!()
    }
}
