#![feature(decl_macro)]
use rocket::{request::Form, response::Redirect, *};
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_contrib::templates::{tera, Template};
use serde_json::json;


#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
fn post_login(mut auth: Auth, form: Form<Login>) -> Redirect {
    auth.login(&form).unwrap();
    Redirect::to("/")
}

#[get("/signup")]
fn get_signup() -> Template {
    let cnxt = tera::Context::new();
    Template::render("signup", cnxt)
}

#[post("/signup", data = "<form>")]
fn post_signup(mut auth: Auth, form: Form<Signup>) -> Redirect {
    auth.signup(&form).unwrap();
    auth.login(&form.into()).unwrap();
    Redirect::to("/")
}

#[get("/")]
fn index(user: Option<User>) -> Template {
    let mut cnxt = tera::Context::new();
    cnxt.insert("user", &user);
    Template::render("index", cnxt)
}

#[get("/logout")]
fn logout(mut auth: Auth) -> &'static str {
    auth.logout().unwrap();
    "You've logged out."
}
#[get("/delete")]
fn delete(mut auth: Auth) -> &'static str {
    auth.delete().unwrap();
    "Your account was deleted."
}

fn main() -> Result<(), Error> {
    let users = Users::open_postgres("host=localhost user=postgres password='password'")?;

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







