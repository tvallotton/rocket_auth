use crate::prelude::try_outcome;
use crate::Auth;

use crate::error::Error;

use rocket::{
    async_trait,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// The `CsrfToken` struct is used to prevent cross
/// site request forgery attacks. When used as a request
/// guard, it retrieves the csrf_token from the users
/// session. If the user does not have a session, one will be
/// created for them. Then the token should be placed in the
/// form or the json being sent by the client so it can be
/// verified in future requests.
/// ```rust
/// #[get("/delete-account")]
/// fn delete_account(token: CsrfToken) -> Template {
///     let cxt = json!({ "csrf_token": token });
///     Template::delete("/delete-account", &cxt)
/// }
/// #[delete("/delete-account")]
/// fn delete_account(auth: Auth<'_>) -> Redirect {
///     auth.delete().await;
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
/// `"DELETE"` methods. This behavior can be configured with [Config](`crate::config::Config`).
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrfToken(pub(crate) String);

#[async_trait]
impl<'r> FromRequest<'r> for CsrfToken {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let outcome = request.guard().await;
        let auth: Auth = try_outcome!(outcome);
        Outcome::Success(auth.csrf_token().await)
    }
}

impl Deref for CsrfToken {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
