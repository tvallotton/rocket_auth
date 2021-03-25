



// use rocket::request::FromRequest;
// use crate::prelude::*;

// use rocket::Request;



// impl<'a, 'r> FromRequest<'a, 'r> for Users {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<User, ()> {
//         let db = request.guard::<Database>()?;
//         request.cookies()
//             .get_private("user_id")
//             .and_then(|cookie| cookie.value().parse().ok())
//             .and_then(|id| db.get_user(id).ok())
//             .or_forward(())
//     }
// }