use crate::prelude::*;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;
use serde_json::json;
use std::time::Duration;
use crate::forms::ValidEmail;

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
    /// Logs in the user through a form. The session is set to expire in one year by default.
    /// for a custom expiration date use [`Auth::login_for`].
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

    pub fn login_for(&mut self, form: &Login, time: Duration) -> Result<()> {
        let key = self.users.login_for(&form, time)?;
        let user = self.users.get_by_email(&form.email)?;
        let session = Session {
            id: user.id,
            email: user.email,
            auth_key: key,
            time_stamp: now() as u32,
        };
        let to_str = format!("{}", json!(session));
        let cookie = Cookie::new("rocket_auth", to_str);
        self.cookies.add_private(cookie);
        Ok(())
    }

    pub fn signup(&mut self, form: &Signup) -> Result<()> {
        self.users.signup(&form)?;
        self.login(&form.into())?;
        Ok(())
    }
    ///
    ///
    /// It allows to know if the current client is authenticated or not.
    /// ```rust
    /// #[get("/am-I-authenticated")]
    /// fn is_auth(auth: Auth) -> &'static str {
    ///     if auth.is_auth() {
    ///         "Yes you are."
    ///     } else {
    ///         "nope."
    ///     }
    /// }
    /// ```
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

    pub fn logout(&self) -> Result<()> {
        let session = self.get_session()?;
        self.users.logout(session)?;
        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        let session = self.get_session()?;
        self.users.delete(session.id)?;
        Ok(())
    }

    pub fn change_password(&self, password: String) -> Result<()> {
        if self.is_auth() {
            let session = self.get_session()?;
            let mut user = self.users.get_by_id(session.id)?;
            user.reset_password(&password)?;
            self.users.modify(user)?;
            Ok(())
        } else {
            raise(ErrorKind::Unauthorized, "Unauthorized.")
        }
    }
    pub fn change_email(&self, email: String) -> Result<()> {
        if self.is_auth() {
            email.is_valid()?;
            let session = self.get_session()?;
            let mut user = self.users.get_by_id(session.id)?;
            user.email = email;
            self.users.modify(user)?;
            Ok(())
        } else {
            raise(ErrorKind::Unauthorized, "Unauthorized.")
        }
    }

    pub fn get_session(&self) -> Result<&Session> {
        let session = self.session.as_ref().ok_or(Error {
            message: "Client is not authenticated".into(),
            kind: ErrorKind::UnauthenticatedClientError,
        })?;
        Ok(session)
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
