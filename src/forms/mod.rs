use crate::prelude::*;
pub use crate::{Login, Signup};
use lazy_static::lazy_static;
use regex::Regex;

const EMAIL_REGEX: &str = r"^[\w\-\.]+@([\w-]+\.)+[\w\-]{2,4}$";

impl Signup {
    pub fn is_valid(&self) -> Result<()> {
        self.password.is_secure()?;
        self.email.is_valid()?;
        Ok(())
    }
}

impl From<&Signup> for Login {
    fn from(form: &Signup) -> Login {
        Login {
            email: form.email.clone(),
            password: form.password.clone(),
        }
    }
}

pub trait ValidEmail {
    fn is_valid(&self) -> Result<()>;
}

pub trait SafePassword {
    // const UPER_CASE: &'static str = "ASDFGHJKLQWERTYUIOPZXCVBNM";
    // const LOWER_CASE: &'static str = "qwertyuiopasdfhjklzxcvbn";
    // const NUMBER: &'static str = "1234567890";
    // const SYMBOLS: &'static str = "~`!@#$%^&*()_-+={[}]|\\:;\"'<,>.?/";

    fn is_secure(&self) -> Result<()>;
}

impl ValidEmail for str {
    fn is_valid(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(EMAIL_REGEX).unwrap();
        }
        if RE.is_match(&self) {
            Ok(())
        } else {
            Err(Error {
                message: format!("'{}' is not a valid email address.", self),
                kind: ErrorKind::FormValidationError,
            })
        }
    }
}

impl SafePassword for str {
    fn is_secure(&self) -> Result<()> {
        if self.len() > 8 {
            Ok(())
        } else {
            raise(
                ErrorKind::UnsafePasswordError,
                "Unsafe password. It must be at least 8 characters long.",
            )
        }
    }
}
