mod user;
pub mod auth;
mod users;


use rand::random;
pub fn rand_string(size: usize) -> String {
    // let dissallowed = ['\\', '"', '{', '}', '(', ')', '`', '\''];
    (0..)
        .map(|_| random::<u8>())
        .filter(|n| 31 < *n && *n < 126)
        .map(|n| char::from(n))
        // .filter(|c| !dissallowed.contains(c))
        .take(size)
        .collect()
}



use crate::Users;
use crate::Session;
impl Users {
    fn is_auth(&self, session: &Session) -> bool {
        let option = self.sess.get(session.id);
        if let Some(auth_key) = option {
            auth_key == session.auth_key
        } else {
            false
        }
    }
}