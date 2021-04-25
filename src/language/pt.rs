
use super::*;

use Error::*;

impl Language {
    pub fn pt_message(error: Error) -> &'static str {
        match error {
            UnsafePasswordTooShort => "A senha deve ter pelo menos 8 caracteres.",
            UnauthenticatedClientError => "O cliente não está autenticado.",
            UnauthorizedError => "Não autorizado.",
            InvalidCredentialsError => "Seu correio eletrônico ou senha está incorreta.",
            UserNotFoundError => "O usuário não foi encontrado.",
            InvalidEmailAddressError => "O correio eletrônico não é válido.",
            EmailAlreadyExists => "Esse email já existe.",
            _ => "Erro interno do servidor."
        }
    }
}
  
  