use std::rc::Rc;

use super::AuthKey;
use super::SessionManager;
use crate::prelude::*;
use chashmap::CHashMap;

#[async_trait]
impl SessionManager for CHashMap<i32, AuthKey> {
    async fn insert(&self, user_id: i32, session_id: &str, time: Duration) -> Result {
        let key = AuthKey {
            expires: time.as_secs() as i64,
            secret: session_id.into(),
        };
        self.insert(user_id, key);
        Ok(())
    }

    async fn remove(&self, id: i32) -> Result {
        self.remove(&id);
        Ok(())
    }

    async fn get(&self, id: i32) -> Option<String> {
        let key = self.get(&id)?;
        Some(key.secret.clone())
    }

    async fn clear_all(&self) -> Result {
        self.clear();
        Ok(())
    }

    async fn clear_expired(&self) -> Result {
        let time = now();
        self.retain(|_, auth_key| auth_key.expires > time);
        Ok(())
    }
}
