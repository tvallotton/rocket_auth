use crate::prelude::*;
use argon2::verify_encoded as verify;
use std::time::Duration;
mod api;



#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub id: u32,
    pub email: String,
    password: String,
    pub is_admin: bool,
}


pub struct Users {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}

