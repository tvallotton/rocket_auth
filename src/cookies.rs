use crate::prelude::*;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use serde_json::{from_str};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The Session guard can be used to retrieve user session data. 
/// Unlike `User`, using session does not verify that the session data is 
/// still valid. Since the client could have logged out, or their session
/// may have expired. The Session guard is intended for purposes where
/// verifying the validity of the session data is unnecessary. 
/// 
/// Note that, 
/// session data is already captured by the [`Auth`] guard and stored in the public `session` field. 
/// So it is not necesarry to use them together. 
pub struct Session {
    /// The user id as it is stored on the database. 
    pub id: i32,
    /// The user email. 
    pub email: String,
    /// An random authentication token key.
    pub auth_key: String,
    /// It represents the Unix time in which the user logged in. It is measured in seconds. 
    pub time_stamp: i32,
}
#[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;
   async fn from_request(request: &'r Request<'_>) -> Outcome<Session, Self::Error> {
        let cookies = request.cookies();

        if let Some(session) = get_session(cookies) {
            Outcome::Success(session)
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                Error::UnauthorizedError
            ))
        }
    }
}

fn get_session(cookies: &CookieJar) -> Option<Session> {
    let session = cookies.get_private("rocket_auth")?;

    from_str(&session.value()).ok()
    
}
