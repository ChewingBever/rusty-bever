#[macro_use]
extern crate diesel;

pub mod auth;
pub mod db;
pub mod errors;
pub(crate) mod schema;

pub use errors::Result;

// Any import defaults are defined here
/// Expire time for the JWT tokens in seconds.
const JWT_EXP_SECONDS: i64 = 600;
/// Amount of bytes the refresh tokens should consist of
const REFRESH_TOKEN_N_BYTES: usize = 64;
/// Expire time for refresh tokens; here: one week
const REFRESH_TOKEN_EXP_SECONDS: i64 = 604800;
