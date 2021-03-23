use crate::prelude::*;
use std::time::Duration;
pub mod default;

#[cfg(feature = "redis-session")]
pub mod redis;


pub trait SessionManager: Send + Sync {
    fn insert(&self, id: u64, key: String) -> Result<()>;
    fn insert_for(&self, id: u64, key: String, time: Duration) -> Result<()>;
    fn remove(&self, id: u64) -> Result<()>;
    fn get(&self, id: u64) -> Option<String>;
    fn flush(&self) -> Result<()>;
    fn clear_expired(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct AuthKey {
    expires: u64,
    secret: String,
}

impl From<String> for AuthKey {
    fn from(secret: String) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret
        }
    }
}

impl From<&str> for AuthKey {
    fn from(secret: &str) -> AuthKey {
        AuthKey {
            expires: 31536000,
            secret: secret.into()
        }
    }
}
