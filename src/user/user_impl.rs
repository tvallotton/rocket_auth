use super::auth::Auth;
use super::rand_string;

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
    /// 
    /// ```rust
    /// # use rocket::{State, post};
    /// # use rocket_auth::{Error, Users};
    /// #[post("/reset-password/<new_password>")]
    /// async fn reset_password(mut user: User, users: &State<Users>, new_password: String) -> Result<(), Error> {
    ///     user.set_password(&new_password);
    ///     users.modify(&user).await?;
    ///     Ok(())
    /// }
    /// ```
    #[throws(Error)]
    pub fn set_password(&mut self, new: &str) {
        crate::forms::is_secure(new)?;
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, salt.as_bytes(), &config).unwrap();
        self.password = hash;
    }

    /// Compares the password of the currently authenticated user with a another password.
    /// Useful for checking password before resetting email/password.
    /// To avoid bruteforcing this function should not be directly accessible from a route.
    /// Additionally, it is good to implement rate limiting on routes using this function.
    #[throws(Error)]
    pub fn compare_password(&self, password: &str) -> bool {
        verify_encoded(&self.password, password.as_bytes())?
    }

    /// This is an accessor function for the private `id` field.
    /// This field is private, so that it is not modified by accident when updating a user.
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
    /// async fn set_email(email: String, auth: Auth<'_>) -> Result<String, Error> {
    ///     let mut user = auth.get_user().await.unwrap();
    ///     user.set_email(&email)?;
    ///     auth.users.modify(&user).await?;
    ///     Ok("Your user email was changed".into())
    /// }
    /// ```
    #[throws(Error)]
    pub fn set_email(&mut self, email: &str) {
        if validator::validate_email(email) {
            self.email = email.to_lowercase();
        } else {
            throw!(Error::InvalidEmailAddressError)
        }
    }
}

use std::fmt::{self, Debug};

impl Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ id: {:?}, email: {:?}, is_admin: {:?}, password: \"*****\" }}",
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<AdminUser, Error> {
        use rocket::outcome::Outcome::*;
        let guard = request.guard().await;
        let auth: Auth = match guard {
            Success(auth) => auth,
            Failure(x) => return Failure(x),
            Forward(x) => return Forward(x),
        };
        if let Some(user) = auth.get_user().await {
            if user.is_admin {
                return Outcome::Success(AdminUser(user));
            }
        }
        Outcome::Failure((Status::Unauthorized, Error::UnauthorizedError))
    }
}

use std::ops::*;
use argon2::verify_encoded;

impl Deref for AdminUser {
    type Target = User;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AdminUser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl std::convert::TryFrom<User> for AdminUser {
    type Error = Error;
    fn try_from(value: User) -> Result<Self> {
        if value.is_admin {
            Ok(AdminUser(value))
        } else {
            Err(Error::UnauthorizedError)
        }
    }
}
