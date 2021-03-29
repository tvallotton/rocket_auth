use super::auth::Auth;
use super::rand_string;
use crate::forms::SafePassword;
use crate::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

impl User {
    /// This method allows to reset the password of a user.
    /// In order for the new password to be saved, it must be passed to a [`Users`] instance.
    /// This function is meant for cases where the user lost their password.
    /// In case the user is authenticated,
    /// you can change it more easily with [`change_password`](`super::auth::Auth::change_password`).
    /// This function will fail in case the password is not secure enough.
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Users};
    /// #[get("/reset-password/<id>/<new_password>")]
    /// fn reset_password(id: u32, new_password: String, users: State<Users>) -> Result<(), Error> {
    ///     let mut user = users.get_by_id(id)?;
    ///     user.reset_password(&new_password);
    ///     users.modify(user)?;
    ///     Ok(())
    /// }
    /// ```

    pub fn reset_password(&mut self, new: &str) -> Result<()> {
        new.is_secure()?;
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.password = hash;
        Ok(())
    }
}

use std::fmt::{self, Debug};

impl Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ id: {:?}, email: {:?}, is_admin: {:?}}}",
            self.id, self.email, self.is_admin
        )
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Error;
    fn from_request(request: &'a Request) -> Outcome<User, Error> {
        let auth: Auth = request.guard()?;
        if let Some(user) = auth.get_user() {
            Outcome::Success(user)
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                Error {
                    kind: ErrorKind::Unauthorized,
                    message: "Unauthorized".into(),
                },
            ))
        }
    }
}
