#![feature(decl_macro)]
use rocket::{request::Form, response::{Redirect, Responder}, *};
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_contrib::{json, json::*};
use rocket_contrib::templates::{tera, Template};



#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
fn post_login(mut auth: Auth, form: Form<Login>) -> Result<Redirect, JsonValue>{
    let result = auth.login(&form);
    if result.is_err() {
        return Err(json!(result));
    }
    Ok(Redirect::to("/"))
}

#[get("/signup")]
fn get_signup() -> Template {
    let cnxt = tera::Context::new();
    Template::render("signup", cnxt)
}

#[post("/signup", data = "<form>")]
fn post_signup(mut auth: Auth, form: Form<Signup>) -> JsonValue {
    json!({
        "signup": auth.signup(&form),
        "login": auth.login(&form.into())
    })
}

#[get("/")]
fn index(user: Option<User>) -> Template {
    let mut cnxt = tera::Context::new();
    cnxt.insert("user", &user);
    Template::render("index", cnxt)
}

#[get("/logout")]
fn logout(mut auth: Auth) -> JsonValue {
    json!(auth.logout())
}
#[get("/delete")]
fn delete(mut auth: Auth) -> JsonValue {
    json!(auth.delete())
}

fn main() -> Result<(), Error> {
    let users = Users::open_sqlite("database.db")?;

    rocket::ignite()
        .mount("/",
            routes![
                index, 
                get_login, 
                post_signup, 
                get_signup, 
                post_login,
                logout, 
                delete],)
        .manage(users)
        .attach(Template::fairing())
        .launch();
    Ok(())
}
