
use super::*;

use Error::*;

impl Language {
    pub fn es_message(error: Error) -> &'static str {
        match error {
            UnsafePasswordTooShort => "La clave debe tener al menos 8 caracteres.",
            UnauthenticatedClientError => "El cliente no esta autentificado.",
            UnauthorizedError => "No autorizado.",
            InvalidCredentialsError => "Su correo electónico o contraseña es incorrecta.",
            UserNotFoundError => "No se encotró el usuario.",
            InvalidEmailAddressError => "Correo inválido.",
            EmailAlreadyExists => "Ese correo ya existe.",
            _ => "Error interno del servidor."
        }
    }
}
  
  