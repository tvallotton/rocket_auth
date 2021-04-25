

use crate::prelude::*;
#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub enum Language {

    EN,
    #[cfg(feature="lang-es")]
    ES,
}

use Language::*;
impl Language {
    fn message(self, error: Error) -> &'static str {
        match self {
            EN => Language::en_message(error),
        }
    }
}


mod en;