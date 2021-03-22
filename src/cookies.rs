use crate::prelude::*;
use rocket::request::FromForm;
use rocket::request::{self, FromRequest, Request};

#[derive(FromForm)]
pub struct Session {
    pub id: u32,
    pub email: String,
    pub auth_key: u32,
    pub time_stamp: u32,
}

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Session, Error> {
        todo!()
    }
}
