use crate::prelude::*;
use rocket::http::Status;
use rocket::request::{self, FromForm, FromRequest, Outcome, Request};

#[derive(FromForm)]
pub struct Session {
    pub id: u64,
    pub email: String,
    pub auth_key: String,
    pub time_stamp: u32,
}

use std::convert::TryInto;

impl<'a, 'r> FromRequest<'a, 'r> for Session {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Session, Error> {
        let mut cookies = request.cookies();
        let id = cookies.get_private("id");
        let email = cookies.get_private("email");
        let auth_key = cookies.get_private("auth_key");
        let time_stamp = cookies.get_private("time_stamp");

        let clear = &[id, email, auth_key, time_stamp]
            .iter()
            .map(|x| x.is_some())
            .fold(true, |x, y| x && y);
        if !clear {
            return Outcome::Failure((
                Status::Unauthorized,
                Error {
                    message: "Request cookies didn't have the requeired fields.".into(),
                    source: None,
                },
            ));
        }
        todo!()
        // return Outcome::Success([id, email, auth_key, time_stamp].into());
    }
}


