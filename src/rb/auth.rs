use crate::errors::AuthError;
use crate::models::User;
use crate::schema::users::dsl as users;
use argon2::verify_encoded;
use diesel::prelude::*;
use diesel::PgConnection;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::HashMap;
use chrono::Utc;

/// Expire time for the JWT tokens in seconds.
const JWT_EXP_SECONDS: i64 = 900;
/// Amount of bytes the refresh tokens should consist of
const REFRESH_TOKEN_N_BYTES: u32 = 64;

pub fn verify_user(conn: &PgConnection, username: &str, password: &str) -> Result<User, AuthError> {
    // TODO handle non-"NotFound" Diesel errors accordingely
    let user = match users::users
        .filter(users::username.eq(username))
        .first::<User>(conn)
    {
        Err(_) => return Err(AuthError::UnknownUser),
        Ok(user) => user,
    };

    match verify_encoded(user.password.as_str(), password.as_bytes()) {
        Ok(true) => Ok(user),
        _ => Err(AuthError::InvalidPassword),
    }
}

struct JWTResponse {
    token: String,
    refresh_token: String
}

pub fn generate_jwt_token(conn: &PgConnection, user: &User) -> JWTResponse {
    // TODO actually use proper secret here
    // TODO don't just unwrap here
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

    // Create the claims
    let mut claims = HashMap::new();
    claims.insert("id", user.id.to_string());
    claims.insert("username", user.username);
    claims.insert("exp", (Utc::now().timestamp() + JWT_EXP_SECONDS).to_string());

    // Sign the claims into a new token
    // TODO don't just unwrap here
    let token = claims.sign_with_key(&key).unwrap();
}
