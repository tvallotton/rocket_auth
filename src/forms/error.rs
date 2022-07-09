use thiserror::Error;

/// The vaidation error
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("The email {0:?} does not belong to a registered user.")]
    UserNotFound(String), 
    #[error("The email address is not valid.")]
    InvalidEmailAddress, 
    #[error("That email {0:?} already exists, try logging in.")]
    EmailAlreadyExists(String), 
    #[error("The password should be at least 8 characters long.")]
    PasswordTooShort,
    #[error("The password should have at least one upper case letter.")]
    PasswordMissingUppercase,
    #[error("The password should have at least one lowercase letter.")]
    PasswordMissingLowercase,
    #[error("The password should have at least one number.")]
    PasswordMissingNumber,
    #[error("Incorrect email or password.")]
    IncorrectPassword, 
}
