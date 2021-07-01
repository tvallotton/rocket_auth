use crate::prelude::*;
pub use crate::{Login, Signup};
use lazy_static::lazy_static;
use regex::Regex;

const EMAIL_REGEX: &str = r"^[\w\-\.]+@([\w-]+\.)+[\w\-]{2,4}$";

impl Signup {
    /// It checks whether the form is valid.
    /// It is not necesay to check if a form is valid when
    /// using [`Auth::signup`](crate::Auth::signup), since that function
    /// does it already.
    pub fn is_valid(&self) -> Result<()> {
        self.password.is_secure()?;
        self.email.is_valid()?;
        Ok(())
    }
}

impl From<Signup> for Login {
    fn from(form: Signup) -> Login {
        Login {
            email: form.email,
            password: form.password,
        }
    }
}

impl<T: Deref<Target = Signup>> From<T> for Login {
    fn from(form: T) -> Login {
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

    fn is_secure(&self) -> Result<()> {
        self.is_long()?;
        self.has_number()?;
        self.has_uppercase()?;
        self.has_lowercase()?;
        Ok(())
    }
    fn has_number(&self) -> Result<()>;
    fn is_long(&self) -> Result<()>;
    fn has_uppercase(&self) -> Result<()>;
    fn has_lowercase(&self) -> Result<()>;
}

impl ValidEmail for str {
    fn is_valid(&self) -> Result<()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(EMAIL_REGEX).unwrap();
        }
        if RE.is_match(&self) {
            Ok(())
        } else {
            Err(Error::InvalidEmailAddressError)
        }
    }
}

impl SafePassword for str {
    fn is_long(&self) -> Result<()> {
        if self.len() > 8 {
            Ok(())
        } else {
            Err(Error::UnsafePasswordTooShort)
        }
    }
    fn has_uppercase(&self) -> Result<()> {
        for c in self.chars() {
            if c.is_uppercase() {
                return Ok(());
            }
        }
        Err(Error::UnsafePasswordHasNoUpper)
    }
    fn has_lowercase(&self) -> Result<()> {
        for c in self.chars() {
            if c.is_lowercase() {
                return Ok(());
            }
        }
        Err(Error::UnsafePasswordHasNoLower)
    }
    fn has_number(&self) -> Result<()> {
        for c in self.chars() {
            if c.is_numeric() {
                return Ok(());
            }
        }
        Err(Error::UnsafePasswordHasNoDigit)
    }
}
