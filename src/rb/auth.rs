use crate::errors::RBError;
use crate::models::{NewRefreshToken, User, NewUser};
use crate::schema::refresh_tokens::dsl as refresh_tokens;
use crate::schema::users::dsl as users;
use argon2::verify_encoded;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{insert_into, PgConnection};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use rand::{thread_rng, Rng};
use serde::Serialize;
use sha2::Sha256;
use std::collections::HashMap;

/// Expire time for the JWT tokens in seconds.
const JWT_EXP_SECONDS: i64 = 900;
/// Amount of bytes the refresh tokens should consist of
const REFRESH_TOKEN_N_BYTES: usize = 64;

pub fn verify_user(conn: &PgConnection, username: &str, password: &str) -> crate::Result<User> {
    // TODO handle non-"NotFound" Diesel errors accordingely
    let user = users::users
        .filter(users::username.eq(username))
        .first::<User>(conn)
        .map_err(|_| RBError::UnknownUser)?;

    // Check if a user is blocked
    if user.blocked {
        return Err(RBError::BlockedUser);
    }

    match verify_encoded(user.password.as_str(), password.as_bytes()) {
        Ok(true) => Ok(user),
        _ => Err(RBError::InvalidPassword),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTResponse {
    token: String,
    refresh_token: String,
}

pub fn generate_jwt_token(conn: &PgConnection, user: &User) -> crate::Result<JWTResponse> {
    // TODO actually use proper secret here
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(b"some-secret").map_err(|_| RBError::JWTCreationError)?;

    // Create the claims
    let mut claims = HashMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("username", user.username.clone());
    claims.insert("admin", user.admin.to_string());
    claims.insert(
        "exp",
        (Utc::now().timestamp() + JWT_EXP_SECONDS).to_string(),
    );

    // Sign the claims into a new token
    let token = claims
        .sign_with_key(&key)
        .map_err(|_| RBError::JWTCreationError)?;

    // Generate a random refresh token
    let mut refresh_token = [0u8; REFRESH_TOKEN_N_BYTES];
    thread_rng().fill(&mut refresh_token[..]);

    // Store refresh token in database
    insert_into(refresh_tokens::refresh_tokens)
        .values(NewRefreshToken {
            token: refresh_token.to_vec(),
            user_id: user.id,
        })
        .execute(conn)
        .map_err(|_| RBError::JWTCreationError)?;

    Ok(JWTResponse {
        token,
        refresh_token: base64::encode(refresh_token),
    })
}

pub fn hash_password(password: &str) -> crate::Result<String> {
    // Generate a random salt
    let mut salt = [0u8; 64];
    thread_rng().fill(&mut salt[..]);

    // Encode the actual password
    let config = argon2::Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).map_err(|_| RBError::PWSaltError)
}

pub fn create_admin_user(conn: &PgConnection, username: &str, password: &str) -> crate::Result<bool> {
    let pass_hashed = hash_password(password)?;
    println!("{}", pass_hashed);
    let new_user = NewUser {
            username: username.to_string(),
            password: pass_hashed,
            admin: true,
        };

    insert_into(users::users)
        .values(&new_user)
        // .on_conflict((users::username, users::password, users::admin))
        // .do_update()
        // .set(&new_user)
        .execute(conn).map_err(|_| RBError::AdminCreationError)?;

    Ok(true)
}
