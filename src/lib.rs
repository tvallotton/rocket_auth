mod cookies;
mod db;
mod error;
mod forms;
mod prelude;
mod session;
mod user;

#[cfg(test)]
mod tests;


use prelude::*;
pub use cookies::Session;
pub use error::Error;
use rocket::FromForm;

pub type Result<T, E = Error> = std::result::Result<T, E>;


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



#[derive(FromForm, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    password: String,
}

#[derive(FromForm, Deserialize, Debug)]
pub struct Signup {
    pub email: String,
    password: String,
}
