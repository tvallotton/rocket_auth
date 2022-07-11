use crate::prelude::*;
use std::time::Duration;
pub mod default;

#[cfg(feature = "redis")]
pub mod redis;

#[async_trait]
pub trait SessionManager: Send + Sync {
    /// This function is available for in-memory session managers that
    /// need to spawn a task to periodically poll sessions and remove them
    /// if they have expired
    async fn init(self);
    /// This method is used to create a session for a logged user.
    async fn create_auth(&self, session_id: &str, user_id: i32, time: Duration) -> Result;
    /// This methid is used to create a session for an unauthenticated user.
    async fn create_unauth(&self, session_id: &str, time: Duration) -> Result;
    /// This is used to retrieve a session.
    async fn get(&self, session_id: &str) -> Option<SessionEntry>;
    /// Destroys a session from a session_id.
    async fn destroy(&self, session_id: &str) -> Option<SessionEntry>;
    /// Destroys every session a user might hold.
    /// This is used to log a user out from all the active sessions.
    async fn destroy_by_user(&self, user_id: i32) -> Result;
    /// Destroys all sessions.
    async fn destroy_all(&self) -> Result;
}

type SessionEntry = Option<i32>;
pub(crate) use None as Unauth;
pub(crate) use Some as Auth;
