use std::fmt::Debug;

use rocket::http::Cookie;
use rocket::http::SameSite;
use std::time::Duration;

/// ```rust
/// let config = Config::new()
///     .require_csrf_token(RequireCsrf::WriteOnly)
///     .same_site_credentials(SameSite::Lax)
///     .session_expiration(Duration::from_secs(15 * 24 * 60.pow(2)));
///
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// defaults to WriteOnly.
    pub require_csrf_token: RequiredCrsf,

    /// defaults to strict
    pub same_site: SameSite,
    /// defaults to true
    pub secure_cookie: bool,
    /// defaults to true
    pub private_session_cookie: bool,
    /// defaults to a week.
    pub session_expiration: Duration,

    pub csrf_token_generation: CsrfGeneration,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            require_csrf_token: RequiredCrsf::WriteOnly,
            same_site: SameSite::Strict,
            secure_cookie: true,
            private_session_cookie: true,
            session_expiration: Duration::from_secs(7 * 24 * 60 * 60),
            csrf_token_generation: CsrfGeneration::PerSession,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CsrfGeneration {
    PerRequest,
    PerSession,
}

#[derive(Debug, Clone)]
pub enum RequiredCrsf {
    /// No authenticated action will require a csrf_token.
    /// Setting this option to never implies that the cookies
    /// containing the session data will be automatically
    /// set to `"strict"`, and that post requests comming from
    /// a different origin will be blocked by default. Beware, not
    /// all [browsers support same-site](https://caniuse.com/same-site-cookie-attribute)
    /// cookies, which would make them
    /// vulnerable to
    Never,
    /// Only `"POST"`, `"PUT"`, `"PATCH"` and `"DELETE"` methods will require
    /// a csrf_token.
    WriteOnly,
    /// All authetincated actions will require a valid
    /// csrf_token.
    Always,
}
