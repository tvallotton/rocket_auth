pub use crate::{Login, Signup};
use lazy_static::lazy_static;
use regex::Regex;

const EMAIL_REGEX: &str = r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$";

impl Signup {
    pub fn is_valid(&self) -> bool {
        self.password_is_secure() && self.is_valid_email()
    }
    fn password_is_secure(&self) -> bool {
        let mut out = self.password.len() > 8;
        // out &= self.is_not_similar_to_email();
        // out &= self.not_common_password();
        out
    }
    
    fn is_valid_email(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(EMAIL_REGEX).unwrap();
        }
        RE.is_match(&self.email)
    }
}
