use crate::prelude::*;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;

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
        self.cookies.add_private(Cookie::new("auth_key", key));
        self.cookies.add_private(Cookie::new("email", user.email));
        self.cookies
            .add_private(Cookie::new("id", format!("{}", user.id)));
        self.cookies
            .add_private(Cookie::new("time_stamp", format!("{}", now())));
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
fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("Error computing SystemTime")
        .unwrap_or(Duration::from_secs(0))
        .as_millis()
}
