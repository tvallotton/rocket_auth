use lazy_static::lazy_static;
use regex::Regex;
use rocket::FromForm;

const EMAIL_REGEX: &str = r"^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$";


#[derive(FromForm, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(FromForm, Debug)]
pub struct Signup {
    pub email: String,
    pub password: String,
}

impl Signup {
    pub fn is_valid(&self) -> bool {
        self.is_valid_password() && self.is_valid_email()
    }
    fn is_valid_password(&self) -> bool {
        self.password.len() > 8
    }
    fn is_valid_email(&self) -> bool {
        lazy_static! {
            static ref re: Regex = Regex::new(EMAIL_REGEX).unwrap();
        }
        re.is_match(&self.email)
    }
}
