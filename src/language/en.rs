
use super::*;

use Error::*;

impl Language {
    pub fn en_message(error: Error) -> &'static str {
        match error {
            UnsafePasswordTooShort => "The password has to be at least 8 characters long.",
            UnauthenticatedClientError => "Client is not authenticated.",
            UnauthorizedError => "Unauthorized.",
            InvalidCredentialsError => "Your email or password is incorrect.",
            UserNotFoundError => "User not found",
            InvalidEmailAddressError => "That email address is not valid.",
            EmailAlreadyExists => "That email already exists. Try logging in.",
            _ => "Internal server error."
        }
    }
}
  
  
    