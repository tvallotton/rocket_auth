use crate::user::rand_string;
use crate::{prelude::*, CsrfToken};
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use serde_json::from_str;

/// The Session guard can be used to retrieve user session data.
/// Unlike `User`, using session does not verify that the session data is
/// still valid. Since the client could have logged out, or their session
/// may have expired. The Session guard is intended for purposes where
/// verifying the validity of the session data is unnecessary.
///
/// Note that, session data is already captured by the [`Auth`](`crate::Auth`)
/// guard and stored in the public [`session`](`crate::Auth`) field.
/// So it is not necessary to use them together.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum Session {
    Authenticated(Authenticated),
    Unauthenticated(Unauthenticated),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct Authenticated {
    /// It represents the Unix time in which the user logged in. It is measured in seconds.
    pub(crate) timestamp: i64,
    /// The user id as it is stored on the database.
    pub(crate) id: i32,
    /// The user email.
    pub(crate) email: String,
    /// a random session identifier.
    pub(crate) session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct Unauthenticated {
    /// a random session identifier.
    pub(crate) session_id: String,
}

impl Session {
    /// returns the csrf token for this session.
    pub(crate) fn csrf_token(&self) -> CsrfToken {
        /// Secret key used to generate csrf tokens for sessions
        static SECRET_KEY: Lazy<String> = Lazy::new(|| rand_string(32));
        let mut seeder = rand_seeder::Seeder::from((self.session_id(), &*SECRET_KEY));

        let crsf_token = seeder
            .make_rng::<SmallRng>()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        CsrfToken(crsf_token)
    }
    /// returns the session identifier. This is used to look up
    /// the user in the session manager.
    pub fn session_id(&self) -> &str {
        match self {
            Self::Unauthenticated(unauth) => &*unauth.session_id,
            Self::Authenticated(auth) => &*auth.session_id,
        }
    }

    #[throws(Error)]
    pub(crate) fn id(&self) -> i32 {
        match self {
            Session::Authenticated(Authenticated { id, .. }) => *id,
            _ => throw!(Error::Unauthorized),
        }
    }
    pub(crate) fn auth(&self) -> Option<&Authenticated> {
        match self {
            Session::Authenticated(auth) => Some(auth),
            _ => None,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn unauth(&self) -> Option<&Unauthenticated> {
        match self {
            Session::Unauthenticated(unauth) => Some(unauth),
            _ => None,
        }
    }

    #[throws(as Option)]
    pub(crate)fn from_cookies(cookies: &CookieJar) -> Session {
        let session = cookies.get_private("rocket_auth")?;
        from_str(session.value()).ok()?
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Session, Self::Error> {
        let cookies = request.cookies();

        if let Some(session) = Self::from_cookies(cookies) {
            Outcome::Success(session)
        } else {
            Outcome::Failure((Status::Unauthorized, Error::Unauthorized))
        }
    }
}
