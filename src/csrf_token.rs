use crate::prelude::try_outcome;
use crate::{cookies::Unauthenticated, Users};
use crate::{Auth, Session::*};

use crate::{error::Error, user::rand_string, Session};
use rocket::State;
use rocket::{
    async_trait,
    form::FromForm,
    http::{hyper::header::REFERER, Cookie, Method::*},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// The `CsrfToken` struct is used to prevent cross
/// site request forgery attacks. When used as a request
/// guard, it sets the csrf_token in the users session.
/// Then the token should be placed in the form or the
/// json being sent by the client so it can be verified in
/// future requests.
/// ```rust
/// #[get("/delete-account")]
/// fn delete_account(token: CsrfToken) -> Template {
///     let cxt = json!({ "csrf_token": token });
///     Template::delete("/delete-account", &cxt)
/// }
/// #[delete("/delete-account")]
/// fn delete_account(user: User) -> Redirect {
///     user.delete().await;
///     Redirect::to(uri!("/"))
/// }
/// ```
/// The template for `"/delete-account"` may look like this:
/// ```html
/// <form method="DELETE" action="/delete-acount">
///     <input type="hidden" name="csrf_token" value="{{ csrf_token }}">
///     <input type="submit" value="delete my account"/>
/// </form>
/// ```
/// The user must first access the form in `"/delete-account"`. This
/// action will register a CSRF token in the users' session data server-side.
/// When the user submits the form, a delete request containing the csrf_token
/// will be sent to the server. Finally, the server will be able to validate if
/// the token sent by the client.
///
/// CSRF tokens are only checked when using the `"POST"`, `"PUT"`, `"PATCH"`, or
/// `"DELETE"` methods.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrfToken(String);

#[async_trait]
impl<'r> FromRequest<'r> for CsrfToken {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //  if user has a session {
        //    set csrf_token for that session
        //  } else {
        //    create session for user
        //    set csrf_token for the new session
        // }
        let outcome = request.guard().await;
        let session: Session = try_outcome!(outcome);
        // let outcome = request.guard().await;
        // let users: State<Users> = try_outcome!(outcome);
        
        // match session {
        //     Authenticated(auth) => {
        //         users.sess.insert(id, key)
        //     }
        // }

        todo!()
    }
}

impl Deref for CsrfToken {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
