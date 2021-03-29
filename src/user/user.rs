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
    ///     user.set_password(&new_password);
    ///     users.modify(&user)?;
    ///     Ok(())
    /// }
    /// ```

    pub fn set_password(&mut self, new: &str) -> Result<()> {
        new.is_secure()?;
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.password = hash;
        Ok(())
    }

    /// This is an accessor function for the private `id` field.
    /// This field is private so it is not modified by accident when updating a user. 
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, User};
    /// #[get("/show-my-id")]
    /// fn show_my_id(user: User) -> String {
    ///     format!("Your user_id is: {}", user.id())
    /// }
    /// ```
    pub fn id(&self) -> u32 {
        self.id
    }
    /// This is an accessor field for the private `email` field. 
    /// This field is private so an email cannot be updated without checking whether it is valid. 
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, User};
    /// #[get("/show-my-email")]
    /// fn show_my_email(user: User) -> String {
    ///     format!("Your user_id is: {}", user.email())
    /// }
    /// ```
    pub fn email(&self) -> &str {
        &self.email
    }

    /// This functions allows to easily modify the email of a user. 
    /// In case the input is not a valid email, it will return an error. 
    /// In case the user corresponds to the authenticated client, it's easier to use [`Auth::change_email`].
    /// ```rust
    /// # #![feature(decl_macro)]
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Auth};
    /// #[get("/set-email/<email>")]
    /// fn show_my_email(email: String, auth: Auth) -> Result<String, Error> {
    ///     let mut user = auth.get_user().unwrap();
    ///     user.set_email(&email)?;
    ///     auth.users.modify(&user)?;
    ///     Ok("Your user email was changed".into())
    /// }
    /// ```
    pub fn set_email(&mut self, email: &str) -> Result<()> {
        use crate::forms::ValidEmail;
        email.is_valid()?;
        self.email = email.into();
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
