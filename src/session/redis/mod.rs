use super::SessionManager;
use crate::prelude::*;

use redis::Client;

impl SessionManager for Client {
    fn insert(&self, id: u32, key: String) -> Result<()> {
        todo!()
    }
    fn insert_for(&self, id: u32, key: String, time: Duration) -> Result<()> {
        todo!()
    }
    fn remove(&self, id: u32) -> Result<()> {
        todo!()
    }
    fn get(&self, id: u32) -> Option<String> {
        todo!()
    }
    fn flush(&self) -> Result<()> {
        todo!()
    }
    fn clear_expired(&self) -> Result<()> {
        todo!()
    }
}
