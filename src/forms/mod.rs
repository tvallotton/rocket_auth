use crate::prelude::*;
pub use error::SignupError;
use validator::validate_email;
use SignupError::*;
mod error;

/// The `Login` form is used along with the [`Auth`] guard to authenticate users.
#[derive(FromForm, Deserialize, Clone, Hash, PartialEq, Eq)]
pub struct Login {
    pub email: String,
    pub(crate) password: String,
}

/// The `Signup` form is used along with the [`Auth`] guard to create new users.
#[derive(FromForm, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Signup {
    pub email: String,
    pub(crate) password: String,
}

impl Signup {
    pub fn validate(&self) -> Result<(), Vec<SignupError>> {
        let password = is_secure(&self.password);
        let email = validate_email(&self.email);
        match (password, email) {
            (Ok(()), false) => Err(vec![InvalidEmailAddressError]),
            (Err(mut errors), false) => {
                errors.push(InvalidEmailAddressError);
                Err(errors)
            }
            (result, _) => result,
        }
    }
}

impl Debug for Signup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Signup {{ email: {:?}, password: \"*****\" }}",
            self.email
        )
    }
}
impl Debug for Login {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Signup {{ email: {:?}, password: \"*****\" }}",
            self.email
        )
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

impl From<Login> for Signup {
    fn from(form: Login) -> Signup {
        Self {
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

pub(crate) fn is_secure(password: &str) -> Result<(), Vec<SignupError>> {
    let mut errors = vec![];
    if is_too_short(password) {
        errors.push(PasswordTooShort)
    }
    if missing_lowercase(password) {
        errors.push(PasswordMissingLowercase);
    }
    if missing_uppercase(password) {
        errors.push(PasswordMissingUppercase);
    }
    if missing_number(password) {
        errors.push(PasswordMissingNumber);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn is_too_short(password: &str) -> bool {
    password.len() < 8
}

fn missing_uppercase(password: &str) -> bool {
    !password.chars().any(char::is_uppercase)
}

fn missing_lowercase(password: &str) -> bool {
    !password.chars().any(char::is_lowercase)
}

fn missing_number(password: &str) -> bool {
    !password.chars().any(char::is_numeric)
}
