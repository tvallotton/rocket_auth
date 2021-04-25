

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
        }
    }
}

mod en;
#[cfg(feature="lang-es")]
mod es;
#[cfg(feature="lang-pt")]
mod pt;