use rocket::{form::*, get, post, response::Redirect, routes, State};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use rusqlite::Connection;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

use std::result::Result;
use std::*;
#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    auth.login(&form).await?;
    Ok(Redirect::to("/"))
}

#[get("/signup")]
async fn get_signup() -> Template {
    Template::render("signup", json!({}))
}

#[post("/signup", data = "<form>")]
async fn post_signup(auth: Auth<'_>, form: Form<Signup>) -> Result<Redirect, Error> {
    auth.signup(&form).await.map_err(|err| dbg!(err))?;
    auth.login(&form.into()).await?;
    Ok(Redirect::to("/"))
}

#[get("/")]
async fn index(user: Option<User>) -> Template {
    Template::render("index", json!({ "user": user }))
}

#[post("/logout")]
async fn post_logout(auth: Auth<'_>) -> Result<Template, Error> {
    auth.logout().await?;
    Ok(Template::render("logout", json!({})))
}

#[post("/delete")]
async fn delete(auth: Auth<'_>) -> Result<Template, Error> {
    auth.delete().await?;
    Ok(Template::render("deleted", json!({})))
}

#[get("/show_all_users")]
async fn show_all_users(
    conn: &State<Arc<Mutex<Connection>>>,
    user: Option<User>,
) -> Result<Template, Error> {
    let users: Vec<_> = conn
        .lock()
        .await
        .prepare_cached("select * from users;")?
        .query_map([], |row| Ok(row.get(1)?))?
        .filter_map(Result::ok)
        .map(|email: String| json!({ "email": email }))
        .collect();
    Ok(Template::render(
        "users",
        json!({"users": users, "user": user}),
    ))
}

#[rocket::launch]
async fn rocket() -> _ {
    let conn = Connection::open("database.db").unwrap();
    let conn = Arc::new(Mutex::new(conn));
    let users: Users = conn.clone().into();
    users.create_table().await.unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_login,
                post_signup,
                post_logout,
                get_signup,
                post_login,
                delete,
                show_all_users,
            ],
        )
        .manage(conn)
        .manage(users)
        .attach(Template::fairing())
}
