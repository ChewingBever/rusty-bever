use ::jwt::SignWithKey;
use argon2::verify_encoded;
use chrono::Utc;
use diesel::{insert_into, prelude::*, PgConnection};
use hmac::{Hmac, NewMac};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{
    db::{
        tokens::{NewRefreshToken, RefreshToken},
        users::{NewUser, User},
    },
    errors::RbError,
    schema::{refresh_tokens::dsl as refresh_tokens, users::dsl as users},
};

pub mod jwt;

pub fn verify_user(conn: &PgConnection, username: &str, password: &str) -> crate::Result<User>
{
    // TODO handle non-"NotFound" Diesel errors accordingely
    let user = users::users
        .filter(users::username.eq(username))
        .first::<User>(conn)
        .map_err(|_| RbError::AuthUnknownUser)?;

    // Check if a user is blocked
    if user.blocked {
        return Err(RbError::AuthBlockedUser);
    }

    match verify_encoded(user.password.as_str(), password.as_bytes()) {
        Ok(true) => Ok(user),
        _ => Err(RbError::AuthInvalidPassword),
    }
}

pub fn hash_password(password: &str) -> crate::Result<String>
{
    // Generate a random salt
    let mut salt = [0u8; 64];
    thread_rng().fill(&mut salt[..]);

    // Encode the actual password
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|_| RbError::Custom("Couldn't hash password."))
}

pub fn create_admin_user(conn: &PgConnection, username: &str, password: &str)
    -> crate::Result<bool>
{
    let pass_hashed = hash_password(password)?;
    let new_user = NewUser {
        username: username.to_string(),
        password: pass_hashed,
        admin: true,
    };

    insert_into(users::users)
        .values(&new_user)
        .on_conflict(users::username)
        .do_update()
        .set(&new_user)
        .execute(conn)
        .map_err(|_| RbError::Custom("Couldn't create admin."))?;

    Ok(true)
}
