//! The db module contains all Diesel-related logic. This is to prevent the various Diesel imports
//! from poluting other modules' namespaces.

pub mod posts;
pub mod sections;
pub mod tokens;
pub mod users;

pub use posts::{NewPost, PatchPost, Post};
pub use sections::{NewSection, Section};
pub use tokens::{NewRefreshToken, RefreshToken};
pub use users::{NewUser, User};
