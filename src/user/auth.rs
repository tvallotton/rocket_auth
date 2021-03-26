use crate::prelude::*;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;
use serde_json::json;

pub struct Auth<'a> {
    pub users: State<'a, Users>,
    pub cookies: Cookies<'a>,
    pub session: Option<Session>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth<'a> {
    type Error = Error;
    fn from_request(req: &'a Request<'r>) -> Outcome<Auth<'a>, Error> {
        let session: Option<Session> = if let Outcome::Success(users) = req.guard() {
            Some(users)
        } else {
            None
        };
        let users: State<Users> = if let Outcome::Success(users) = req.guard() {
            users
        } else {
            return Outcome::Failure((Status::Unauthorized, Error {
                message: "Attempted to load Users, but it was not being managed. Possible fix:  add `.manage(users)` to your rocket apllication.".into(),
                kind: ErrorKind::UnmanagedStateError,
            }));
        };

        Outcome::Success(Auth {
            users: users,
            cookies: req.cookies(),
            session: session,
        })
    }
}

impl<'a> Auth<'a> {
    pub fn login(&mut self, form: &Login) -> Result<()> {
        let key = self.users.login(&form)?;
        let user = self.users.get_by_email(&form.email)?;
        let session = Session {
            id: user.id,
            email: user.email,
            auth_key: key,
            time_stamp: now() as u32,
        };
        let to_str = format!("{}", json!(session));
        self.cookies.add_private(Cookie::new("rocket_auth", to_str));
        Ok(())
    }

    pub fn signup(&mut self, form: &Signup) -> Result<()> {
        self.users.signup(&form)?;
        self.login(&form.into())?;
        Ok(())
    }
    pub fn is_auth(&self) -> bool {
        if let Some(session) = &self.session {
            self.users.is_auth(session)
        } else {
            false
        }
    }
    pub fn get_user(&self) -> Option<User> {
        let id = self.session.clone()?.id;
        if let Ok(user) = self.users.get_by_id(id) {
            println!("{:?}", user);
            Some(user)
        } else {
            None
        }
    }

    pub fn logout() -> Result<()> {
        todo!()
    }
    pub fn delete() -> Result<()> {
        todo!()
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("Error computing SystemTime")
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}
