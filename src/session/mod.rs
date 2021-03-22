




use crate::prelude::*;


pub trait SessionManager: Send + Sync {
    fn insert(&self, id: u32, key: &str) -> Result<()>;
    fn remove(&self, id: u32) -> Result<()>;
}
