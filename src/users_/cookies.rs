use crate::prelude::*;
use rocket::request::FromForm;
use rocket::request::{self, FromRequest, Request};

#[derive(FromForm)]
pub struct CookieUser {
    pub id: u32,
    pub email: String,
    pub auth_key: u32,
    pub time_stamp: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for CookieUser {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CookieUser, Error> {
        
        todo!()
    }
}
