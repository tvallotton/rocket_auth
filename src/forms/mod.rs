pub use crate::{Login, Signup};
use lazy_static::lazy_static;
use regex::Regex;
use crate::prelude::*;

const EMAIL_REGEX: &str = r"^[\w\-\.]+@([\w-]+\.)+[\w\-]{2,4}$";

impl Signup {
    pub fn is_valid(&self) -> Result<()> {
        self.password_is_secure()?;
        self.is_valid_email()?;
        Ok(())
    }

    fn password_is_secure(&self) -> Result<()> {
        if self.password.len() > 8 {
            Ok(())
        } else {
            Err(Error {
                message: "Passwords have to be at least 8 characters long.".into(),
                kind: ErrorKind::FormValidationError,
            })
        }
    }
    
    fn is_valid_email(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(EMAIL_REGEX).unwrap();
        }
        if RE.is_match(&self.email) {
            Ok(())
        } else {
            Err(Error { 
                message: format!("'{}' is not a valid email address.", self.email),
                kind: ErrorKind::FormValidationError
            })
        }
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
