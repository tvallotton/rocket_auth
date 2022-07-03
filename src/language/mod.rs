use rocket_lang::*; 
use crate::prelude::*;
use crate::Error; 


fn message(error: Error, lang: LangCode) -> &'static str {
    match lang {
        _ => en::message(error), 
        
    }
}


mod en; 
mod es; 
mod pt; 





