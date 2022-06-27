use super::{SessionEntry, Auth, Unauth};
use super::SessionManager;
use crate::prelude::*;
use redis::{AsyncCommands, Client};
use rocket::serde::json::from_str;

#[async_trait]
impl SessionManager for Client {
    async fn init(self) {}

    async fn create_auth(&self, session_id: &str, user_id: i32, time: Duration) -> Result {
        let mut cnn = self.get_async_connection().await?;
        cnn.set_ex(session_id, user_id, time.as_secs() as usize)
            .await?;
        cnn.lpush(user_id, session_id).await?;
        cnn.expire(user_id, time.as_secs() as usize).await?;
        Ok(())
    }
    async fn create_unauth(&self, session_id: &str, time: Duration) -> Result {
        let mut cnn = self.get_async_connection().await?;
        cnn.set_ex(session_id, "", time.as_secs() as usize).await?;
        Ok(())
    }

    async fn destroy(&self, session_id: &str) -> Option<SessionEntry> {
        let mut cnn = self.get_async_connection().await.ok()?;
        let user_id = cnn.del(session_id).await.ok()?;
        Some(Auth(user_id))
    }

    async fn get(&self, session_id: &str) -> Option<SessionEntry> {
        let mut cnn = self.get_async_connection().await.ok()?;
        let s: String = cnn.get(session_id).await.ok()?;
        from_str(&s).ok()
    }
    async fn destroy_by_user(&self, user_id: i32) -> Result {
        let mut cnn = self.get_async_connection().await?;
        let sessions: Vec<String> = cnn.lrange(user_id, 1, -1).await?;
        for session_id in sessions {
            cnn.del(session_id).await?;
        }
        Ok(())
    }

    async fn destroy_all(&self) -> Result {
        let mut cnn = self.get_async_connection().await?;
        redis::Cmd::new()
            .arg("FLUSHDB")
            .query_async::<_, ()>(&mut cnn)
            .await?;
        Ok(())
    }
}

