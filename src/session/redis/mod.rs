use super::SessionManager;
use crate::prelude::*;


use redis::{Commands, Client};

const YEAR_IN_SECS: usize = 365 * 60 * 60 * 24;

impl SessionManager for Client {
    fn insert(&self, id: i32, key: String) -> Result<()> {
        let mut cnn = self.get_connection()?;
        println!("connected;\n\n");
        cnn.set_ex(id, key, YEAR_IN_SECS)?;
        Ok(())
    }
    fn insert_for(&self, id: i32, key: String, time: Duration) -> Result<()> {
        let mut cnn = self.get_connection()?;
        cnn.set_ex(id, key, time.as_secs() as usize)?;
        Ok(())
    }
    fn remove(&self, id: i32) -> Result<()> {
        let mut cnn = self.get_connection()?;
        cnn.del(id)?;
        Ok(())
    }
    fn get(&self, id: i32) -> Option<String> {
        let mut cnn = self.get_connection().ok()?;
        let key = cnn.get(id).ok()?;
        Some(key)
    }
    fn clear_all(&self) -> Result<()> {
        let mut cnn = self.get_connection()?;
        redis::Cmd::new().arg("FLUSHDB").execute(&mut cnn);
        Ok(())
    }
    fn clear_expired(&self) -> Result<()> {
        Ok(())
    }
}


