use crate::prelude::*;
use std::time::Duration;
pub mod default;

#[cfg(feature = "redis")]
pub mod redis;

#[async_trait]
pub trait SessionManager: Send + Sync {
    async fn insert(&self, id: i32, key: String) -> Result;
    async fn insert_for(&self, id: i32, key: String, time: Duration) -> Result;
    async fn remove(&self, id: i32) -> Result;
    async fn get(&self, id: i32) -> Option<String>;
    async fn clear_all(&self) -> Result;
    async fn clear_expired(&self) -> Result;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthKey {
    expires: i64,
    secret: String,
}

impl From<String> for AuthKey {
    fn from(secret: String) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret,
        }
    }
}

impl From<&str> for AuthKey {
    fn from(secret: &str) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret: secret.into(),
        }
    }
}
