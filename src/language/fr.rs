
use super::*;

use Error::*;

impl Language {
    pub fn fr_message(error: Error) -> &'static str {
        match error {
            UnsafePasswordTooShort => "Le mot de passe doit faire au moins 8 caractères.",
            UnauthenticatedClientError => "Client non authentifié.",
            UnauthorizedError => "Non autorisé.",
            InvalidCredentialsError => "Email ou mot de passe incorrect.",
            UserNotFoundError => "Cet utilisateur n'existe pas.'",
            InvalidEmailAddressError => "Email adresse invalide.",
            EmailAlreadyExists => "Cette email adresse existe déjà.",
            _ => "Erreur interne au serveur.",
        }
    }
}
