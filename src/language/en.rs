pub use crate::prelude::*;
use std::borrow::Cow;

pub fn message(error: &Error) -> Vec<Cow<'static, str>> {
    match error {
        Error::Unauthorized => vec!["Unauthorized, try logging in.".into()],
        Error::Forbidden => {
            vec!["Forbidden. You do not have permission to access this resource.".into()]
        }
        Error::Validation(errors) => errors.iter().map(validation).collect(),
        _ => vec!["Internal server error.".into()],
    }
}

fn validation(error: &ValidationError) -> Cow<'static, str> {
    match error {
        PasswordTooShort => "The password should be at least 8 characters long.".into(),
        InvalidEmailAddress => "The email address is not valid.".into(),
        PasswordMissingNumber => "The password should have at least one number.".into(),
        IncorrectPassword => "Incorrect email or password.".into(),
        PasswordMissingUppercase => {
            "The password should have at least one upper case letter.".into()
        }
        PasswordMissingLowercase => {
            "The password should have at least one lowercase letter.".into()
        }
        EmailAlreadyExists(email) => {
            format!("The email {email:?} already exists, try logging in.").into()
        }
        UserNotFound(email) => {
            format!("The email {email:?} does not belong to a registered user.").into()
        }
    }
}
