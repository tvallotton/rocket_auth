use crate::prelude::*; 
use Error::*; 

pub fn message(error: Error) -> &'static str {
    
    match error {
        // Error:: => "The password has to be at least 8 characters long.",
        // Error::UnauthenticatedClientError => "Client is not authenticated.",
        // Error::UnauthorizedError => "Unauthorized.",
        // Error::InvalidCredentialsError => "Your email or password is incorrect.",
        // Error::UserNotFoundError => "User not found",
        // Error::InvalidEmailAddressError => "That email address is not valid.",
        // Error::EmailAlreadyExists => "That email already exists, try logging in.",
        _ => "Internal server error.",
    }
}
