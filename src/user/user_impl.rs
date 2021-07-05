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
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Users};
    /// #[get("/reset-password/<id>/<new_password>")]
    /// async fn reset_password(id: i32, new_password: String, users: &State<Users>) -> Result<(), Error> {
    ///     let mut user = users.get_by_id(id).await?;
    ///     user.set_password(&new_password);
    ///     users.modify(&user).await?;
    ///     Ok(())
    /// }
    /// ```
    #[throws(Error)]
    pub fn set_password(&mut self, new: &str) {
        new.is_secure()?;
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.password = hash;
    }

    /// This is an accessor function for the private `id` field.
    /// This field is private so it is not modified by accident when updating a user.
    /// ```rust
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, User};
    /// #[get("/show-my-id")]
    /// fn show_my_id(user: User) -> String {
    ///     format!("Your user_id is: {}", user.id())
    /// }
    /// ```
    pub fn id(&self) -> i32 {
        self.id
    }
    /// This is an accessor field for the private `email` field.
    /// This field is private so an email cannot be updated without checking whether it is valid.
    /// ```rust
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
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Auth};
    /// #[get("/set-email/<email>")]
    /// async fn show_my_email(email: String, auth: Auth<'_>) -> Result<String, Error> {
    ///     let mut user = auth.get_user().await.unwrap();
    ///     user.set_email(&email)?;
    ///     auth.users.modify(&user).await?;
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<User, Error> {
        use rocket::outcome::Outcome::*;
        let guard = request.guard().await;
        let auth: Auth = match guard {
            Success(auth) => auth,
            Failure(x) => return Failure(x),
            Forward(x) => return Forward(x),
        };
        if let Some(user) = auth.get_user().await {
            Outcome::Success(user)
        } else {
            Outcome::Failure((Status::Unauthorized, Error::UnauthorizedError))
        }
    }
}
