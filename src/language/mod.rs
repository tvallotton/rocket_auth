

use crate::prelude::*;

impl Error {
    pub fn message(self, lang: Language) -> &'static str {
        lang.message(self)
    }
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub enum Language {
    EN,
    #[cfg(feature="lang-es")]
    ES,
    #[cfg(feature="lang-pt")]
    PT,
    #[cfg(feature="lang-fr")]
    FR,
}

use Language::*;
impl Language {
    fn message(self, error: Error) -> &'static str {
        match self {
            EN => Language::en_message(error),
            #[cfg(feature="lang-es")]
            ES => Language::es_message(error),
            #[cfg(feature="lang-pt")]
            PT => Language::pt_message(error),
            #[cfg(feature="lang-fr")]
            FR => Language::fr_message(error),
        }
    }
}

mod en;
#[cfg(feature="lang-es")]
mod es;
#[cfg(feature="lang-pt")]
mod pt;
#[cfg(feature="lang-fr")]
mod fr;
