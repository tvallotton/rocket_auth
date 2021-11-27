use super::SessionManager;
use crate::prelude::*;

use redis::{Client, Commands};

const YEAR_IN_SECS: usize = 365 * 60 * 60 * 24;

#[async_trait]
impl SessionManager for Client {
    async fn insert(&self, id: i32, key: String) -> Result {
        let mut cnn = self.get_connection()?;
        cnn.set_ex(id, key, YEAR_IN_SECS)?;
        Ok(())
    }

    async fn insert_for(&self, id: i32, key: String, time: Duration) -> Result {
        let mut cnn = self.get_connection()?;
        cnn.set_ex(id, key, time.as_secs() as usize)?;
        Ok(())
    }

    async fn remove(&self, id: i32) -> Result {
        let mut cnn = self.get_connection()?;
        cnn.del(id)?;
        Ok(())
    }

    async fn get(&self, id: i32) -> Option<String> {
        let mut cnn = self.get_connection().ok()?;
        cnn.get(id).ok()
    }

    async fn clear_all(&self) -> Result {
        let mut cnn = self.get_connection()?;
        redis::Cmd::new().arg("FLUSHDB").execute(&mut cnn);
        Ok(())
    }

    async fn clear_expired(&self) -> Result {
        Ok(())
    }
}
