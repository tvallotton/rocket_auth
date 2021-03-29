use crate::forms::ValidEmail;
use crate::prelude::*;
use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use rocket::State;
use serde_json::json;
use std::time::Duration;

/// The [`Auth`] guard allows to log in, log out, sign up, modify, and delete the currently (un)authenticated user.
/// For more information see [`Auth`]. Because of rust's ownership rules, you may not retrieve both `rocket::http::Cookies` and the [`Auth`] guard
/// simultaneously. However, retrieveng cookies is not needed since `Auth` stores them in the public field [`Auth::cookies`].
///  A working example:
/// ```rust,no_run
/// #![feature(proc_macro_hygiene, decl_macro)]
/// use rocket::{get, post, routes, request::Form};
/// use rocket_auth::{Users, Error, Auth, Signup, Login};
///
/// #[post("/signup", data="<form>")]
/// fn signup(form: Form<Signup>, mut auth: Auth) {
///     // users are automatically logged in after signing up.
///     auth.signup(&form);
/// }
///
/// #[post("/login", data="<form>")]
/// fn login(form: Form<Login>, mut auth: Auth) {
///     auth.login(&form);
/// }
///
/// #[get("/logout")]
/// fn logout(mut auth: Auth) {
///     auth.logout();
/// }
///
/// fn main() -> Result<(), Error>{
///     let users = Users::open_sqlite("mydb.db")?;
///
///     rocket::ignite()
///         .mount("/", routes![signup, login, logout])
///         .manage(users)
///         .launch();
///     Ok(())
/// }
/// ```
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
    /// Logs in the user through a parsed form or json. The session is set to expire in one year by default.
    /// for a custom expiration date use [`Auth::login_for`].
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{get, post, request::Form};
    /// # use rocket_auth::{Auth, Login};
    /// #[post("/login", data="<form>")]
    /// fn login(form: Form<Login>, mut auth: Auth) {
    ///     auth.login(&form);
    /// }
    /// ```
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

    /// Logs a user in for the specified period of time.
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{post, request::Form};
    /// # use rocket_auth::{Login, Auth};
    /// # use std::time::Duration;
    /// #[post("/login", data="<form>")]
    /// fn login(form: Form<Login>, mut auth: Auth) {
    ///     let one_hour = Duration::from_secs(60 * 60);
    ///     auth.login_for(&form, one_hour);
    /// }
    /// ```
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

    /// Creates a new user from a form or a json.
    /// The cliend will be automatically logged in.
    /// Their session will be set to expire in a year.
    /// In order to customize the expiration date use [`signup_for`].
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{post, request::Form};
    /// # use rocket_auth::{Auth, Login};
    /// # use std::time::Duration;
    /// #[post("/login", data="<form>")]
    /// fn login(form: Form<Login>, mut auth: Auth) {
    ///     let one_hour = Duration::from_secs(60 * 60);
    ///     auth.login_for(&form, one_hour);
    /// }
    /// ```
    pub fn signup(&mut self, form: &Signup) -> Result<()> {
        self.users.signup(&form)?;
        self.login(&form.into())?;
        Ok(())
    }

    /// Creates a new user from a form or a json.
    /// The session will last the specified period of time. 
    /// In order to customize the expiration date use [`signup_for`].
    /// ```rust
    /// #![feature(decl_macro)]
    /// # use rocket::{post, request::Form};
    /// # use rocket_auth::{Auth, Login};
    /// # use std::time::Duration;
    /// #[post("/login", data="<form>")]
    /// fn login(form: Form<Login>, mut auth: Auth) {
    ///     let one_hour = Duration::from_secs(60 * 60);
    ///     auth.login_for(&form, one_hour);
    /// }
    /// ```
    pub fn signup_for(&mut self, form: &Signup, time: Duration) -> Result<()> {
        self.users.signup(&form)?;
        self.login_for(&form.into(), time)?;
        Ok(())
    }

    ///
    ///
    /// It allows to know if the current client is authenticated or not.
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{get};
    /// # use rocket_auth::{Auth};
    /// #[get("/am-I-authenticated")]
    /// fn is_auth(auth: Auth) -> &'static str {
    ///     if auth.is_auth() {
    ///         "Yes you are."
    ///     } else {
    ///         "nope."
    ///     }
    /// }
    /// # fn main() {}
    /// ```
    pub fn is_auth(&self) -> bool {
        if let Some(session) = &self.session {
            self.users.is_auth(session)
        } else {
            false
        }
    }

    /// It retrieves the current logged user. 
    /// This will fail on expired sessions. 
    /// ```
    /// # #![feature(decl_macro)]
    /// # use rocket::get;
    /// # use rocket_auth::Auth;
    /// #[get("/display-me")]
    /// fn display_me(auth: Auth) -> String {
    ///     format!("{:?}", auth.get_user())
    /// }
    /// ```
    pub fn get_user(&self) -> Option<User> {
        if !self.is_auth() {
            return None;
        }
        let id = self.session.as_ref()?.id;
        if let Ok(user) = self.users.get_by_id(id) {
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
        if self.is_auth() {
            let session = self.get_session()?;
            self.users.delete(session.id)?;
            Ok(())
        } else {
            Err(Error {
                message: "Client is not logged in.".into(),
                kind: ErrorKind::UnauthenticatedClientError,
            })
        }
    }

    pub fn change_password(&self, password: String) -> Result<()> {
        if self.is_auth() {
            let session = self.get_session()?;
            let mut user = self.users.get_by_id(session.id)?;
            user.reset_password(&password)?;
            self.users.modify(&user)?;
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
            self.users.modify(&user)?;
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
