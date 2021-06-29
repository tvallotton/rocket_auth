
use rocket::{response::{Redirect, Responder}, *};
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket::serde::json::{Json};
use rocket_dyn_templates::{Template, Metadata};
use serde_json::json;


#[get("/login")]
async fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login(mut auth: Auth, form: Form<Login>) -> Result<Redirect, JsonValue>{
    let result = auth.login(&form);
    if result.is_err() {
        return Err(result);
    }
    Ok(Redirect::to("/"))
}

#[get("/signup")]
async fn get_signup() -> Template {
    let cnxt = tera::Context::new();
    Template::render("signup", cnxt)
}

#[post("/signup", data = "<form>")]
async fn post_signup(mut auth: Auth, form: Form<Signup>) -> Result<(), &str> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    
}

#[get("/")]
fn index(user: Option<User>) -> Template {
    let mut cnxt = tera::Context::new();
    cnxt.insert("user", &user);
    Template::render("index", cnxt)
}

#[get("/logout")]
async fn logout(mut auth: Auth) -> JsonValue {
    auth.logout().await
}
#[get("/delete")]
fn delete(mut auth: Auth) -> JsonValue {
    json!(auth.delete().map_err(|s|s.message(Language::ES)))
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
