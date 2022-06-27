use std::rc::Rc;

use super::SessionManager;
use super::{Auth, SessionEntry, Unauth};
use crate::prelude::*;
use chashmap::CHashMap;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    expires: i64,
    secret: SessionEntry,
}
#[derive(Default)]
pub struct DefaultManager(CHashMap<String, Entry>);

#[async_trait]
impl SessionManager for DefaultManager {
    async fn create_auth(&self, session_id: &str, user_id: i32, time: Duration) -> Result {
        let entry = Entry {
            expires: time.as_secs() as i64,
            secret: Auth(user_id),
        };
        self.0.insert(session_id.to_owned(), entry);
        Ok(())
    }
    async fn create_unauth(&self, session_id: &str, time: Duration) -> Result {
        let entry = Entry {
            expires: time.as_secs() as i64,
            secret: Unauth,
        };
        self.0.insert(session_id.to_owned(), entry);
        Ok(())
    }

    async fn destroy(&self, session_id: &str) -> Option<SessionEntry> {
        Some(self.0.remove(session_id)?.secret)
    }

    async fn get(&self, session_id: &str) -> Option<SessionEntry> {
        let key = self.0.get(session_id)?;

        Some(key.secret.clone())
    }

    async fn destroy_all(&self) -> Result {
        self.0.clear();
        Ok(())
    }

    async fn destroy_by_user(&self, user_id: i32) -> Result {
        todo!()
    }

    async fn init(self) {
        tokio::spawn(async move {
            loop {
                let time = now();
                self.0.retain(|_, auth_key| auth_key.expires > time);
                sleep(Duration::from_secs(24 * 60 * 60)).await;
            }
        });
    }
}
