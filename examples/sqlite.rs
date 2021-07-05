use std::*;
use rocket::{form::*, get, post, response::Redirect, routes, State};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use serde_json::json;
use sqlx::*;
use std::result::Result;
use tokio::sync::*;
#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login(mut auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    auth.login(&form).await?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
async fn get_signup() -> Template {
    Template::render("signup", json!({}))
}

#[post("/signup", data = "<form>")]
async fn post_signup(mut auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    
    Ok(Redirect::to("/"))
}

#[get("/")]
async fn index(user: Option<User>) -> Template {
    Template::render("index", json!({ "user": user }))
}

#[get("/logout")]
fn logout(mut auth: Auth<'_>) -> Result<Template, Error> {
    auth.logout()?;
    Ok(Template::render("logout", json!({})))
}
#[get("/delete")]
async fn delete(mut auth: Auth<'_>) -> Result<Template, Error> {
    auth.delete().await?;
    Ok(Template::render("deleted", json!({})))
}

#[get("/show_all_users")]
async fn show_all_users(conn: &State<std::sync::Arc<Mutex<SqliteConnection>>>, user: Option<User>) -> Result<Template, Error> {
    
    let users: Vec<User> = query_as("select * from users;")
        .fetch_all(&mut *conn.lock().await)
        .await?;
    println!("{:?}", users);
    Ok(Template::render("users", json!({"users": users, "user": user})))
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = SqliteConnection::connect("database.db").await?;
    let conn: sync::Arc<Mutex<_>> = sync::Arc::new(conn.into());
    let users: Users = conn.clone().into();

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_login,
                post_signup,
                get_signup,
                post_login,
                logout,
                delete,
                show_all_users
            ],
        )
        .manage(conn)
        .manage(users)
        .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
    Ok(())
}
