



use crate::prelude::*;
use super::rand_string;

impl User {

    /// This method allows to reset the password of a user. 
    /// In order for the new password to be saved, it must be passed to a [`Users`] instance.
    /// This function is meant for cases where the user lost their password. 
    /// In case the user is authenticated,
    /// you can change it more easily with [`change_password`](`super::auth::Auth::change_password`).
    /// ```rust 
    /// #[get("/reset-password/<id>/<new_password>")]
    /// fn reset_password(id: u32, new_password: String, users: State<Users>) -> Result<()> {
    ///     let mut user = users.get_by_id(id);
    ///     user.reset_password();
    ///     users.modify(user)?;
    ///     Ok(())
    /// }
    /// ```

    pub fn reset_password(&mut self, new: &str) {
        let password = new.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.password = hash;
    }
}



use std::fmt::{Debug, self};


impl Debug for User {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User {{ id: {:?}, email: {:?}, is_admin: {:?}}}", self.id, self.email, self.is_admin)
    }
}