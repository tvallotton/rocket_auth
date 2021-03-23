




use crate::prelude::*;
use super::AuthKey;
use chashmap::CHashMap;
use super::SessionManager;

impl SessionManager for CHashMap<u32, AuthKey> {

    // Unnecesary Result
    fn insert(&self, id: u32, key: String) -> Result<()> {
        self.insert(id, key.into());
        Ok(())
    }
    // Unnecesary Result
    fn remove(&self, id: u32) -> Result<()> {
        self.remove(&id);
        Ok(())
    }

    fn get(&self, id: u32) -> Option<String> {
        let key = self.get(&id)?;
        Some(key.secret.clone())
    }

    fn flush(&self) -> Result<()> {
        self.clear();
        Ok(())
    }

    fn insert_for(&self, id: u32, key: String, time: Duration) -> Result<()> {
        let key = AuthKey {
            expires: time.as_secs(),
            secret: key,
        };
        self.insert(id, key);
        Ok(())
    }
    fn clear_expired(&self) -> Result<()> {
        let time = now()?;

        for x in self.filter(||)
        todo!()
    }
}




use std::time::{SystemTime, UNIX_EPOCH};
fn now() -> Result<u128> {
    use crate::error::SetErrorMessage;
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("Error computing SystemTime")?
        .as_millis())
}
