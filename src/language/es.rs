use crate::prelude::*;
use std::borrow::Cow;
use ValidationError::*;

pub fn message(error: &Error) -> Vec<Cow<'static, str>> {
    match error {
        Error::Unauthorized => vec!["No autorizado, intenta registrarte.".into()],
        Error::Forbidden => {
            vec!["No permitido, No tienes permiso para acceder a este recurso.".into()]
        }
        Error::Validation(errors) => errors.iter().map(validation).collect(),
        _ => vec!["Error interno del servidor.".into()],
    }
}

fn validation(error: &ValidationError) -> Cow<'static, str> {
    match error {
        PasswordTooShort => "La contraseña debe tener al menos 8 letras.".into(),
        InvalidEmailAddress => "El correo electrónico no es valido.".into(),
        PasswordMissingNumber => "La contraseña debe tener al menos un número.".into(),
        IncorrectPassword => "El correo o la contraseña es incorrecta.".into(),
        PasswordMissingUppercase => "La contraseña debe tener al menos una letra mayúscula.".into(),
        PasswordMissingLowercase => "La contraseña debe tener al menos una letra minúscula.".into(),
        EmailAlreadyExists(email) => {
            format!("El correo electrónico {email:?} ya existe, intenta ingresar.").into()
        }
        UserNotFound(email) => {
            format!("El correo {email:?} no pertenece a ningún usuario registrado.").into()
        }
    }
}
