//! rocket_auth provides a ready-to-use  backend agnostic API for authentication management.
//! It supports connections for SQLite and Postgresql. It lets you create, delete, and authenticate users.
//! The available features are:
//! * `sqlite-db`: for interacting with a SQLite database using [`sqlx`].
//! * `postgres-db`: for interacting with a Postgresql database with [`sqlx`].
//! * `tokio-postgres-db`: for interacting with a Postgresql database with [`tokio_postgres`].
//! * `redis-session`: for storing sessions on a redis server using [`redis`].
//!
//! By default this crate stores sessions on a concurrent hashmap.
//! As a result, sessions will only be stored as long as the rocket application runs uninterrupted.
//! In order to store persistent sessions, it is recommended to connect the [`Users`](`Users::open_redis`) instance to a [redis server](https://redis.io/) .
//! This requires the `redis-session` feature to be enabled.
//!
//! `rocket_auth` uses private cookies to store session data.
//! This means that in order for cookies to be properly decrypted between launches, a `secret_key` must be set.
//! For more information visit rocket's [configuration guide](https://rocket.rs/v0.4/guide/configuration/).
//!
//!
//!
//!
//!
//! To use `rocket_auth` include it as a dependency in your Cargo.toml file:
//! ```ini
//! [dependencies.rocket_auth]
//! version = "0.3.0"
//! features = ["sqlite-db"]
//! ```
//! # Quick overview
//! This crate provides two guards:
//! * [`Auth`]: manages authentication.
//! * [`Session`]: retrieves session data from client cookies.
//! * [`User`]: It restricts content, so it can be viewed by authenticated clients only.
//!
//! It also includes two structs to be parsed from forms and json data:
//! * [`Signup`]: used to create new users.
//! * [`Login`]: used to authenticate users.
//!
//! Finally it has two structures for queries:
//! * [`Users`]: it allows to query users to the database.
//! * [`User`]: it is the response of a query.
//!

//! The [`Auth`] guard allows to log in, log out, sign up, modify, and delete the currently (un)authenticated user.
//! For more information see [`Auth`]. Because of rust's ownership rules, you may not retrieve both `rocket::http::Cookies` and the [`Auth`] guard
//! simultaneously. However, retrieveng cookies is not needed since `Auth` stores them in the public field [`Auth::cookies`].
//!  A working example:
//! ```rust,no_run
//! use rocket::{get, post, form::Form, routes};
//! use rocket_auth::{Users, Error, Auth, Signup, Login};
//!
//! #[post("/signup", data="<form>")]
//! async fn signup(form: Form<Signup>, mut auth: Auth<'_>) -> Result<&'static str, Error> {
//!     auth.signup(&form).await?;
//!     auth.login(&form.into());
//!     Ok("You signed up.")
//! }
//!
//! #[post("/login", data="<form>")]
//! async fn login(form: Form<Login>, mut auth: Auth<'_>) -> Result<&'static str, Error>{
//!     auth.login(&form).await?;
//!     Ok("You're logged in.")
//! }
//!
//! #[get("/logout")]
//! fn logout(mut auth: Auth<'_>) {
//!     auth.logout();
//! }
//! #[tokio::main]
//! async fn main() -> Result<(), Error>{
//!     let users = Users::open_sqlite("mydb.db").await?;
//!
//!     rocket::build()
//!         .mount("/", routes![signup, login, logout])
//!         .manage(users)
//!         .launch();
//!     Ok(())
//! }
//! ```
//!
//! ## Users struct
//! The [`Users`] struct administers interactions with the database.
//! It lets you query, create, modify and delete users.
//! Unlike the [`Auth`] guard, a [`Users`] instance can manage any user in the database.
//! Note that the [`Auth`] guards includes a `Users` instance stored on the public `users` field.
//! So it is not necessary to retrieve Users when using `Auth`.
//! A simple example of how to query a user with the [`Users`] struct:
//!
//! ```rust
//! # use rocket::{get, State};
//! # use serde_json::json;
//! use rocket_auth::Users;
//!
//! #[get("/see-user/<id>")]
//! async fn see_user(id: i32, users: &State<Users>) -> String {
//!     let user = users.get_by_id(id).await.unwrap();
//!     format!("{}", json!(user))
//! }
//! # fn main() {}
//! ```
//!
//! A [`Users`] instance can be constructed by connecting it to the database with the methods [`open_sqlite`](Users::open_sqlite),
//! [`open_postgres`](Users::open_postgres). Furthermore, it can be constructed from a working connection.
//!
//!
//! ## User guard
//! The `User` guard can be used to restrict content so it can only be viewed by authenticated users.
//! Additionally, yo can use it to render special content if the client is authenticated or not.
//! ```rust
//! # use rocket::*;
//! # use rocket_auth::User;
//! #[get("/private-content")]
//! fn private_content(user: User) -> &'static str {
//!     "If you can see this, you are logged in."
//! }
//!
//! #[get("/special-content")]
//! fn special_content(option: Option<User>) -> String {
//!     if let Some(user) = option {
//!         format!("hello, {}.", user.email())
//!     } else {
//!         "hello, anonymous user".into()
//!     }
//! }
//! ```
//!

mod cookies;
mod db;
mod error;
mod forms;
pub mod prelude;
mod session;
mod user;

#[cfg(test)]
mod tests;

use prelude::*;
use rocket::FromForm;

// pub use language::Language;
pub use crate::user::auth::Auth;
pub use cookies::Session;
pub use error::Error;

/// The `User` guard can be used to restrict content so it can only be viewed by authenticated users.
/// ```rust
/// #
/// # use rocket::{get};
/// # use rocket_auth::User;
/// #[get("/private-content")]
/// fn private_content(user: User) -> &'static str {
///     "If you can see this, you are logged in."
/// }
/// # fn main() {}
/// ```
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct User {
    id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    password: String,
    pub is_admin: bool,
}

/// The `Users` struct is used to query users from the database, as well as to create, modify and delete them.
pub struct Users {
    conn: Box<dyn DBConnection>,
    sess: Box<dyn SessionManager>,
}

/// The `Login` form is used along with the [`Auth`] guard to authenticate users.
#[derive(FromForm, Deserialize, Debug, Clone)]
pub struct Login {
    pub email: String,
    password: String,
}

/// The `Signup` form is used along with the [`Auth`] guard to create new users.

#[derive(FromForm, Deserialize, Debug, Clone)]
pub struct Signup {
    pub email: String,
    password: String,
}
