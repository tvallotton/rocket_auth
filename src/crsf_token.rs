use crate::cookies::Unauthenticated;
use crate::Session::*;

use serde::{Serialize, Deserialize};
use crate::{error::Error, user::rand_string, Session};
use std::ops::Deref; 
use rocket::{
    async_trait,
    http::{hyper::header::REFERER, Method::*},
    request::{FromRequest, Outcome},
    Request,
};

/// The `CrsfToken` struct is used to prevent cross 
/// site request forgery attacks. When used as a request
/// guard, it sets the crsf_token in the users session.
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
///     <input type="hidden" name="crsf_token" value="{{ crsf_token }}">
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
        let string = rand_string(32);
        
        
        match request.method() {
            Delete | Post | Put | Patch => (), 
            _ => ()
        }
        match request.guard().await {
            Outcome::Success(Authenticated(auth)) => {}
            Outcome::Success(Unauthenticated(unauth)) => (),
            _ => (),
        }
        todo!()
    }
}


impl Deref for CsrfToken {
    type Target = str; 
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}