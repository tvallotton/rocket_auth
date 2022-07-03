use thiserror::Error;

/// The vaidation error
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("The email address is not valid.")]
    InvalidEmailAddressError, 
    #[error("That email already exists, try logging in.")]
    EmailAlreadyExists, 
    #[error("The password should be at least 8 characters long.")]
    PasswordTooShort,
    #[error("The password should have at least one upper case letter.")]
    PasswordMissingUppercase,
    #[error("The password should have at least one lowercase letter.")]
    PasswordMissingLowercase,
    #[error("The password should have at least one number.")]
    PasswordMissingNumber,
}
