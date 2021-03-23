mod cookies;
mod db;
mod error;
mod forms;
mod prelude;
mod session;
mod user;

#[cfg(test)]
mod tests;

pub use cookies::Session;
pub use error::Error;
pub use user::{User, Users};

use std::error::Error as ErrorTrait;

pub type Result<T, E = Box<dyn ErrorTrait>> = std::result::Result<T, E>;
