



use rocket::request::FromRequest;
use crate::prelude::*;
use rocket::State;
use rocket::Request;
use rocket::request::Outcome;
use rocket::http::Status;

pub struct Auth<'a> {
    users: State<'a, Users>,
    session: Option<Session<'a>>,
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth<'a> {
    type Error = Error;
    fn from_request(req: &'a Request<'r>) -> Outcome<Auth<'a>, Error> {
        let session: Option<Session<'a>> = if let Outcome::Success(users) = req.guard() {
            Some(users)
        } else {
            None
        };
        let users: State<Users> = if let Outcome::Success(users) = req.guard() {
            users
        } else {
            return Outcome::Failure((Status::Unauthorized, Error {
                message: "Attempted to load Users, but it was not being managed. Possible fix:  add `.manage(users)` to your rocket apllication.".into(),
                kind: ErrorKind::UnmanagedStateError,
            }));
        };

    
        Outcome::Success(
            Auth {
                users: users,
                session: session,
            }
        )
    }
}

impl<'a> Auth<'a> {

    pub fn login(form: impl Deref<Target=Login>) -> Result<()> {
        todo!()
    }

    pub fn signup(form: impl Deref<Target=Login>) -> Result<()> {
        todo!()
    }

    pub fn logout() -> Result<()> {
        todo!()
    }
    pub fn delete() -> Result<()> {
        todo!()
    }

    
}