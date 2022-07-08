use rocket_lang::*; 
use crate::prelude::*;
use crate::Error; 





const fn unauthorized() -> &'static str {
    "unauthorized"
}



// fn message(error: ValidationError, lang: LangCode) -> &'static str {
//     match lang {
//         _ => en::message(error), 
//         Pt => pt::message(error), 
        
//     }
// }


mod en; 
mod es; 
mod pt; 





