#[macro_use]
extern crate diesel;

pub mod auth;
pub mod errors;
mod models;
pub(crate) mod schema;

pub use errors::Result;
