pub mod views;
pub mod forms;
pub mod models;
pub mod cookies;
use views::*;
use rocket::{
    routes,
    Route,
};



pub fn urls() -> Vec<Route> {
    routes![
        post_login,
        post_signup,
        get_login,
        get_signup,
        logout,
        see_all_users,
    ]
}