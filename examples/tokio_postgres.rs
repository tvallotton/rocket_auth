use rocket::{form::*, get, post, response::Redirect, routes, State};
use rocket_auth::{prelude::Error, *};
use rocket_dyn_templates::Template;
use serde_json::json;
use std::convert::TryInto;
use tokio_postgres::{connect, Client, NoTls};

use std::result::Result;
use std::sync::Arc;
use std::*;
#[get("/login")]
fn get_login() -> Template {
    Template::render("login", json!({}))
}

#[post("/login", data = "<form>")]
async fn post_login(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    result?;
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
async fn show_all_users(conn: &State<Arc<Client>>, user: Option<User>) -> Result<Template, Error> {
    let users: Vec<User> = conn
        .query("select * from users;", &[])
        .await?
        .into_iter()
        .flat_map(TryInto::try_into)
        .collect();

    Ok(Template::render(
        "users",
        json!({"users": users, "user": user}),
    ))
}

#[rocket::launch]
async fn rocket() -> _ {
    let (client, conn) = connect("host=localhost user=tomasvallotton", NoTls)
        .await
        .unwrap();
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            panic!("{}", e);
        }
    });
    let client = Arc::new(client);
    let users: Users = client.clone().into();
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
        .manage(client)
        .manage(users)
        .attach(Template::fairing())
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     use tokio_postgres::NoTls;
//     let (client, conn) = connect("host=localhost user=tomasvallotton", NoTls).await?;
//     let client = sync::Arc::new(client);
//     let users: Users = client.clone().into();

// tokio::spawn(async move {
//     if let Err(e) = conn.await {
//        panic!("{e}")
//     }
// });
//     users.create_table().await?;
//     let _ = rocket::build()
//         .mount(
//             "/",
//             routes![
//                 index,
//                 get_login,
//                 post_signup,
//                 get_signup,
//                 post_login,
//                 logout,
//                 delete,
//                 show_all_users
//             ],
//         )
//         .manage(client)
//         .manage(users)
//         .attach(Template::fairing())
//         .launch()
//         .await
//         .unwrap();
//     Ok(())
// }
