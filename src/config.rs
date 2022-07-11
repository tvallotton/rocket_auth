
use derive_builder::Builder;
use rocket::fairing::{self, Fairing, Info, Kind};

use rocket::request::{FromRequest, Outcome};
use rocket::{async_trait, Build, Rocket, State};
use rocket::{response, Data};
use rocket::{Request};



use std::fmt::{self, Debug};
use std::time::Duration;

use crate::Error;

/// ```rust
/// let config = Config::new()
///     .require_csrf_token(RequireCsrf::WriteOnly)
///     .session_expiration(Duration::from_secs(15 * 24 * 60i32.powi(2)));
///     .error_response(|err, req| {
///         /* custom responder implementation for rocket_auth::Error */
///     });
/// ```
#[derive(Clone, Builder)]
#[builder(pattern = "owned")]
pub struct Config {
    /// defaults to WriteOnly.
    pub require_csrf_token: RequiredCsrf,
    /// defaults to one week.
    pub session_expiration: Duration,
    pub(crate) error_response:
        Option<for<'r> fn(Error, &'r Request<'_>) -> response::Result<'static>>,
}

struct PrivConfig(Config);

impl Config {
    pub(crate) const DEFAULT: &'static Config = &Config {
        require_csrf_token: RequiredCsrf::WriteOnly,
        session_expiration: Duration::from_secs(7 * 24 * 60 * 60),
        error_response: None,
    };
    /// This will return the `Config` struct from a request, if no Config
    /// structure was set, then the result will be the default value for
    /// `Config`
    pub(crate) fn from_request<'r>(req: &'r Request) -> &'r Config {
        req.local_cache(Config::default)
    }
}
#[async_trait]
impl Fairing for Config {
    fn info(&self) -> Info {
        Info {
            name: "Rocket auth config",
            kind: Kind::Request | Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let config = PrivConfig(self.clone());
        Ok(rocket.manage(config))
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        // Initialize the local cache for `Config`
        let _ = req
            .guard::<&Config>()
            .await
            .map(|config| req.local_cache(|| config.clone()));
    }
}
#[async_trait]
impl<'r> FromRequest<'r> for &'r Config {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<&'r Config, Self::Error> {
        let outcome = request
            .guard::<&State<PrivConfig>>()
            .await
            .map(|priv_config| &priv_config.0);

        match outcome {
            Outcome::Success(_) => outcome,
            _ => Outcome::Success(Config::DEFAULT),
        }
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("require_csrf_token", &self.require_csrf_token)
            .field("session_expiration", &self.session_expiration)
            .field(
                "error_response",
                &self.error_response.map(|_| "|Error, &Request| -> Result"),
            )
            .finish()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::DEFAULT.clone()
    }
}

#[derive(Debug, Clone)]
pub enum RequiredCsrf {
    /// No authenticated action will require a csrf_token.
    /// Setting this option to never implies that the cookies
    /// containing the session data will be automatically
    /// set to `"strict"`, and that post requests comming from
    /// a different origin will be blocked by default. Beware, not
    /// all [browsers support same-site](https://caniuse.com/same-site-cookie-attribute)
    /// cookies, which would make users of these browsers
    /// vulnerable to cross site request forgery attacks.
    Never,
    /// Only `"POST"`, `"PUT"`, `"PATCH"` and `"DELETE"` methods will require
    /// a csrf_token. This is the default behavior.
    WriteOnly,
    /// All authetincated actions will require a valid
    /// csrf_token.
    Always,
}
