use std::borrow::Cow;
use crate::Error;
use rocket_lang::*;

pub fn messages(error: &Error, lang: LangCode) -> Vec<Cow<'static, str>> {
    match lang {
        Es => es::message(error), 
        _ => en::message(error),
    }
}


mod en;
mod es;
// mod pt;
