use crate::prelude::*;
use rocket::http::{Cookies, Status};
use rocket::request::{FromRequest, Outcome, Request};
use serde_json::{from_str};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// The Session guard can be used to retrieve user session data. 
/// Unlike Users, using session does not verify that the session data is 
/// still valid. Since the client could have logged out, or their session
/// may have expired. The Session guard is intended for purposes where
/// verifying the validity of the session data is unnecessary. 
/// 
/// Note that, 
/// session data is already captured by the [`Auth`] guard and stored in the public `session` field. 
/// So it is not necesarry to use them together. 
pub struct Session {
    pub id: u32,
    pub email: String,
    pub auth_key: String,
    pub time_stamp: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<Session, Self::Error> {
        let mut cookies = request.cookies();

        if let Some(session) = get_session(&mut cookies) {
            Outcome::Success(session)
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                Error {
                    message: "To view this page you must be authenticated.".into(),
                    kind: ErrorKind::ClientSessionError,
                },
            ))
        }
    }
}

fn get_session(cookies: &mut Cookies) -> Option<Session> {
    let session = cookies.get_private("rocket_auth")?;

    from_str(&session.value()).ok()
    
}

// type OptionCookie<'a> = Option<rocket::http::Cookie<'a>>;

// fn get_fields<'a>(cookies: &mut Cookies) -> [OptionCookie<'a>; 4] {
//     let id = cookies.get_private("id");
//     let email = cookies.get_private("email");
//     let auth_key = cookies.get_private("auth_key");
//     let time_stamp = cookies.get_private("time_stamp");
//     let x = [id, email, auth_key, time_stamp];
//     println!("{:?}", x);
//     x
// }

// fn are_all_some(array: &[OptionCookie]) -> bool {
//     array.iter().map(|x| x.is_some()).fold(true, |x, y| x && y)
// }

// fn session<'a>([id, email, auth_key, time_stamp]: [OptionCookie; 4]) -> Option<Session> {
//     let result = id?.value().parse();
//     let id;
//     if let Ok(id_) = result {
//         id = id_;
//     } else {
//         return None;
//     }
//     let result = email?.value().parse();
//     let email;
//     if let Ok(email_) = result {
//         email = email_;
//     } else {
//         return None;
//     }
//     let result = auth_key?.value().parse();
//     let auth_key;
//     if let Ok(auth_key_) = result {
//         auth_key = auth_key_;
//     } else {
//         return None;
//     }
//     let result = time_stamp?.value().parse();
//     let time_stamp;
//     if let Ok(time_stamp_) = result {
//         time_stamp = time_stamp_;
//     } else {
//         return None;
//     }
//     Some(Session {
//         id,
//         email,
//         auth_key,
//         time_stamp,
//     })
// }

// use std::time::{SystemTime, UNIX_EPOCH};
// fn now() -> u64 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .msg("Error computing SystemTime")
//         .unwrap_or(Duration::from_secs(0))
//         .as_secs()
// }
