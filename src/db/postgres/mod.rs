use crate::prelude::*;
mod sql;
use tokio_postgres::Client;
use std::convert::{TryInto, TryFrom};

impl DBConnection for Client {
    fn init(&self) -> Result<()> {
        println!("INITIALIZING");
        futures::executor::block_on(
            self.execute(sql::CREATE_TABLE, &[])
        )?;
        Ok(())
    }
    fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        futures::executor::block_on(
            self.execute(sql::INSERT_USER, &[&email, &hash, &is_admin])
        )?;
        Ok(())
    }
    fn update_user(&self, user: User) -> Result<()> {
        futures::executor::block_on(
            self.execute(sql::UPDATE_USER, &[&user.email, &user.password, &user.is_admin])
        )?;
        Ok(())
    }
    fn delete_user_by_id(&self, user_id: u32) -> Result<()> {
        futures::executor::block_on(
            self.execute(sql::REMOVE_BY_ID, &[&user_id])
        )?;
        Ok(())
    }
    fn delete_user_by_email(&self, email: &str) -> Result<()> {
        futures::executor::block_on(
            self.execute(sql::REMOVE_BY_EMAIL, &[&email])
        )?;
        Ok(())
    }
    fn get_user_by_id(&self, user_id: u32) -> Result<User> {
        let user = futures::executor::block_on(
            self.query_one(sql::SELECT_BY_ID, &[&user_id])
        )?;
        user.try_into()
    }


    fn get_user_by_email(&self, email: &str) -> Result<User> {
                let user = futures::executor::block_on(
            self.query_one(sql::SELECT_BY_EMAIL, &[&email])
        )?;
        user.try_into()
    }
}



impl TryFrom<tokio_postgres::Row> for User {
    type Error = Error;
    fn try_from(row: tokio_postgres::Row) -> Result<User> {
        Ok(User {
            id: row.get(0),
            email: row.get(1),
            password: row.get(2), 
            is_admin: row.get(3),
        })   
    }
    
}