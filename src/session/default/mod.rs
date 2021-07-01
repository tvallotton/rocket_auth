use super::AuthKey;
use super::SessionManager;
use crate::prelude::*;
use chashmap::CHashMap;

impl SessionManager for CHashMap<i32, AuthKey> {
    #[throws(Error)]
    fn insert(&self, id: i32, key: String) {
        self.insert(id, key.into());
    }

    #[throws(Error)]
    fn remove(&self, id: i32) {
        self.remove(&id);
    }

    fn get(&self, id: i32) -> Option<String> {
        let key = self.get(&id)?;
        Some(key.secret.clone())
    }

    #[throws(Error)]
    fn clear_all(&self) {
        self.clear();
    }

    #[throws(Error)]
    fn insert_for(&self, id: i32, key: String, time: Duration) {
        let key = AuthKey {
            expires: time.as_secs(),
            secret: key,
        };
        self.insert(id, key);
    }

    #[throws(Error)]
    fn clear_expired(&self) {
        let time = now()? as u64;
        self.retain(|_, auth_key| auth_key.expires > time);
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
#[throws(Error)]
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
}
