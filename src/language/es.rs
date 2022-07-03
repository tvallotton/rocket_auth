use super::*;
use Error::*;

pub fn message(error: Error) -> &'static str {
    match error {
        UnsafePasswordTooShort => "La clave debe tener al menos 8 caracteres.",
        Unauthenticated => "El cliente no esta autentificado.",
        Unauthorized => "No autorizado.",
        InvalidCredentialsError => "Su correo electónico o contraseña es incorrecta.",
        UserNotFoundError => "No se encotró el usuario.",
        InvalidEmailAddressError => "Correo inválido.",
        EmailAlreadyExists => "Ese correo ya existe.",
        _ => "Error interno del servidor.",
    }
}
