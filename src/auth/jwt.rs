use chrono::Utc;
use diesel::PgConnection;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::{
    db,
    errors::{RbError, RbResult},
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
    user: &db::User,
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
        &db::NewRefreshToken {
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
        // If we fail to block the user, the end user must know
        if let Err(err) = db::users::block(conn, token_entry.user_id) {
            return Err(err);
        }

        return Err(RbError::AuthDuplicateRefreshToken);
    }

    // Now we check if the token has already expired
    let cur_time = Utc::now().naive_utc();

    if token_entry.expires_at < cur_time {
        return Err(RbError::AuthTokenExpired);
    }

    // We update the last_used_at value for the refresh token
    db::tokens::update_last_used_at(conn, &token_entry.token, cur_time)?;

    generate_jwt_token(conn, jwt, &user)
}
