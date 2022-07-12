use rocket::local::asynchronous::Client;
use rocket::serde::json::Json;
use rocket::{delete, get, post, response::Redirect, routes, State};
use rocket_auth::{prelude::Error, *};

use serde_json::{json, Value};
use sqlx::*;
use std::result::Result;
use std::*;

#[post("/login", data = "<form>")]
async fn login(auth: Auth<'_>, form: Json<Login>) -> Result<Value, Error> {
    auth.login(&form).await?;
    Ok(json!({ "status": "success" }))
}

#[post("/signup", data = "<form>")]
async fn signup(auth: Auth<'_>, form: Json<Signup>) -> Result<Value, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    Ok(json!({ "status": "success" }))
}

#[get("/")]
async fn index(user: Option<User>) -> Value {
    json!({ "data": user })
}
#[get("/csrf_token")]
async fn csf_token(token: CsrfToken) -> Value {
    json!({ "csrf_token": token })
}

#[post("/logout")]
async fn logout(auth: Auth<'_>) -> Result<Value, Error> {
    auth.logout().await?;
    Ok(json!({ "status": "success" }))
}

#[delete("/delete")]
async fn delete(auth: Auth<'_>) -> Result<Value, Error> {
    auth.delete().await?;
    Ok(json!({ "status": "success" }))
}

pub async fn client(users: Users, config: Option<Config>) -> Client {
    let rocket = rocket::build()
        .mount(
            "/",
            routes![login, signup, index, logout, delete, csf_token],
        )
        .manage(users)
        .attach(config.unwrap_or_default());
    Client::tracked(rocket).await.unwrap()
}
