use argon2::verify_encoded;
use diesel::PgConnection;
use rand::{thread_rng, Rng};

use crate::{
    db,
    errors::{RbError, RbResult},
};

pub fn verify_user(conn: &PgConnection, username: &str, password: &str) -> RbResult<db::User>
{
    // TODO handle non-"NotFound" Diesel errors accordingely
    let user = db::users::find_by_username(conn, username).map_err(|_| RbError::AuthUnknownUser)?;

    // Check if a user is blocked
    if user.blocked {
        return Err(RbError::AuthBlockedUser);
    }

    match verify_encoded(user.password.as_str(), password.as_bytes()) {
        Ok(true) => Ok(user),
        _ => Err(RbError::AuthInvalidPassword),
    }
}

pub fn hash_password(password: &str) -> RbResult<String>
{
    // Generate a random salt
    let mut salt = [0u8; 64];
    thread_rng().fill(&mut salt[..]);

    // Encode the actual password
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_| RbError::Custom("Couldn't hash password."))
}
