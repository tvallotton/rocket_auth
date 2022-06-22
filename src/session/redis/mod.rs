use super::SessionManager;
use crate::prelude::*;

use redis::{AsyncCommands, Client};

#[async_trait]
impl SessionManager for Client {
    async fn insert(&self, user_id: i32, session_id: &str, time: Duration) -> Result {
        let mut cnn = self.get_async_connection().await?;
        cnn.set_ex(session_id, user_id, time.as_secs() as usize)
            .await?;
        Ok(())
    }

    async fn remove(&self, id: i32) -> Result {
        let mut cnn = self.get_async_connection().await?;
        cnn.del(id).await?;
        Ok(())
    }

    async fn get(&self, id: i32) -> Option<String> {
        let mut cnn = self.get_async_connection().await.ok()?;
        cnn.get(id).await.ok()
    }

    async fn clear_all(&self) -> Result {
        let mut cnn = self.get_async_connection().await?;
        redis::Cmd::new()
            .arg("FLUSHDB")
            .query_async::<_, ()>(&mut cnn)
            .await?;
        Ok(())
    }

    async fn clear_expired(&self) -> Result {
        Ok(())
    }
}
