



use crate::prelude::*;
use argon2::verify_encoded as verify;
use std::time::Duration;
use super::rand_string;

impl User {
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