use super::models::User;
use rocket::FromForm;
// use lazy_static::lazy_static;



#[derive(FromForm, Debug)]
pub struct Login {
    pub email: String,
    pub password: String
}

#[derive(FromForm, Debug)]
pub struct Signup {
    pub email: String,
    pub password: String,
}

// lazy_static! {

// c

impl Signup {
    fn is_valid(&self) -> bool {
        self.is_valid_password() && self.is_valid_email()
    }
    fn is_valid_password(&self) -> bool {
        self.password.len() > 8
    }
    fn is_valid_email(&self) -> bool {
        true
    }
}


// #[derive(FromForm)]
// pub struct CookieUser {
//     pub id: u32,
//     pub email: String,
//     pub auth_key: u32,
//     pub time_stamp: u32,
// }


impl From<Login> for User {
    fn from(form: Login) -> User {
        User {
            id: 0,
            email: form.email,
            password: form.password,
            is_admin: false,
            auth_key: None,
        }
    }
} 

// use rocket::request::FromRequest;
// impl<'a, 'r> FromRequest<'a, 'r> for CookieUser {
//     type Error = ApiKeyError;

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         let keys: Vec<_> = request.headers().get("x-api-key").collect();
//         match keys.len() {
//             0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
//             1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
//             1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
//             _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
//         }
//     }
// }


