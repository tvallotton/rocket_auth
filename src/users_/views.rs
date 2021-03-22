#![allow(dead_code)]
use crate::prelude::*;

use super::forms::{Login, Signup};
use super::models::{User, Users};
// use rocket::request::FormParseError;
use rocket::response::Redirect;
use rocket::{
    get, post,
    request::Form,
    State,
};
use rocket_contrib::templates::Template;
#[post("/signup", data = "<form>")]
pub fn post_signup(form: Form<Signup>, users: State<Users>) -> Result<Redirect> {
    users.signin(form)?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
pub fn get_signup() -> Template {
    Template::render("users/signup", json!({}))
}

#[get("/login")]
pub fn get_login() -> Template {
    Template::render("users/login", json!({}))
}

#[post("/login", data = "<form>")]
pub fn post_login(form: Form<Login>, users: State<Users>) -> Result<Redirect> {
    users.login(&form)?;
    Ok(Redirect::to("/"))
    
}

#[get("/logout")]
pub fn logout() -> Template {
    todo!()
}

#[post("/create_admin", data = "<form>")]
pub fn create_admin(form: Form<Signup>, users: State<Users>) -> Result<&'static str> {
    users.signin(form)?;
    Ok("Exito")
}


#[get("/see_all")]
pub fn see_all_users(users: State<Users>) -> Result<String> {
    let db = users.conn.lock()?;

    let mut stmt = db.prepare("SELECT * FROM users;")?;
    let users: Vec<Result<User, rusqlite::Error>> = stmt.query_map(params![], |row| {
        Ok(User {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
            is_admin: row.get(3)?,
            auth_key: row.get(4)?,
        })
    })?.collect();

    Ok(format!("{:?}", users))
}
