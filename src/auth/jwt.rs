use chrono::Utc;
use diesel::{prelude::*, PgConnection};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{
    db,
    db::{tokens::NewRefreshToken, users::User},
    errors::{RbError, RbResult},
    schema::{refresh_tokens::dsl as refresh_tokens, users::dsl as users},
    RbJwtConf,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTResponse
{
    token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims
{
    pub id: uuid::Uuid,
    pub username: String,
    pub admin: bool,
    pub exp: i64,
}

pub fn generate_jwt_token(
    conn: &PgConnection,
    jwt: &RbJwtConf,
    user: &User,
) -> RbResult<JWTResponse>
{
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt.key.as_bytes())
        .map_err(|_| RbError::Custom("Couldn't create Hmac key."))?;

    let current_time = Utc::now();

    // Create the claims
    let claims = Claims {
        id: user.id,
        username: user.username.clone(),
        admin: user.admin,
        exp: current_time.timestamp() + jwt.refresh_token_expire,
    };

    // Sign the claims into a new token
    let token = claims
        .sign_with_key(&key)
        .map_err(|_| RbError::Custom("Couldn't sign JWT."))?;

    // Generate a random refresh token
    let mut refresh_token = vec![0u8; jwt.refresh_token_size];
    thread_rng().fill(&mut refresh_token[..]);

    let refresh_expire =
        (current_time + chrono::Duration::seconds(jwt.refresh_token_expire)).naive_utc();

    // Store refresh token in database
    db::tokens::create(
        conn,
        &NewRefreshToken {
            token: refresh_token.to_vec(),
            user_id: user.id,
            expires_at: refresh_expire,
        },
    )?;

    Ok(JWTResponse {
        token,
        refresh_token: base64::encode(refresh_token),
    })
}

pub fn refresh_token(
    conn: &PgConnection,
    jwt: &RbJwtConf,
    refresh_token: &str,
) -> RbResult<JWTResponse>
{
    let token_bytes =
        base64::decode(refresh_token).map_err(|_| RbError::AuthInvalidRefreshToken)?;

    // First, we request the token from the database to see if it's really a valid token
    let (token_entry, user) =
        db::tokens::find_with_user(conn, &token_bytes).ok_or(RbError::AuthInvalidRefreshToken)?;

    // If we see that the token has already been used before, we block the user.
    if token_entry.last_used_at.is_some() {
        let target = users::users.filter(users::id.eq(token_entry.user_id));
        diesel::update(target)
            .set(users::blocked.eq(true))
            .execute(conn)
            .map_err(|_| RbError::Custom("Couldn't block user."))?;

        return Err(RbError::AuthDuplicateRefreshToken);
    }

    // Now we check if the token has already expired
    let cur_time = Utc::now().naive_utc();

    if token_entry.expires_at < cur_time {
        return Err(RbError::AuthTokenExpired);
    }

    // We update the last_used_at value for the refresh token
    let target = refresh_tokens::refresh_tokens.filter(refresh_tokens::token.eq(token_entry.token));
    diesel::update(target)
        .set(refresh_tokens::last_used_at.eq(cur_time))
        .execute(conn)
        .map_err(|_| RbError::Custom("Couldn't update last used time."))?;

    generate_jwt_token(conn, jwt, &user)
}
