//! Handles refresh token-related database operations.

use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{refresh_tokens, refresh_tokens::dsl::*},
};

/// A refresh token as stored in the database
#[derive(Queryable, Serialize)]
pub struct RefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub last_used_at: Option<chrono::NaiveDateTime>,
}

/// A new refresh token to be added into the database
#[derive(Deserialize, Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "refresh_tokens"]
pub struct PatchRefreshToken
{
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub last_used_at: Option<chrono::NaiveDateTime>,
}

pub fn get(conn: &PgConnection, offset_: u32, limit_: u32) -> RbResult<Vec<RefreshToken>>
{
    Ok(refresh_tokens
        .offset(offset_.into())
        .limit(limit_.into())
        .load(conn)
        .map_err(|_| RbError::DbError("Couldn't query tokens."))?)
}

pub fn create(conn: &PgConnection, new_token: &NewRefreshToken) -> RbResult<RefreshToken>
{
    Ok(insert_into(refresh_tokens)
        .values(new_token)
        .get_result(conn)
        .map_err(|_| RbError::DbError("Couldn't insert refresh token."))?)

    // TODO check for conflict?
}

pub fn update(
    conn: &PgConnection,
    token_: &[u8],
    patch_token: &PatchRefreshToken,
) -> RbResult<RefreshToken>
{
    Ok(diesel::update(refresh_tokens.filter(token.eq(token_)))
        .set(patch_token)
        .get_result(conn)
        .map_err(|_| RbError::DbError("Couldn't update token."))?)
}

pub fn delete(conn: &PgConnection, token_: &[u8]) -> RbResult<()>
{
    diesel::delete(refresh_tokens.filter(token.eq(token_)))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't delete token."))?;

    Ok(())
}

/// Returns the token & user data associated with the given refresh token value.
///
/// # Arguments
///
/// * `conn` - database connection to use
/// * `token_val` - token value to search for
pub fn find_with_user(
    conn: &PgConnection,
    token_: &[u8],
) -> Option<(RefreshToken, super::users::User)>
{
    // TODO actually check for errors here
    refresh_tokens
        .inner_join(crate::schema::users::dsl::users)
        .filter(token.eq(token_))
        .first::<(RefreshToken, super::users::User)>(conn)
        .map_err(|_| RbError::DbError("Couldn't get refresh token & user."))
        .ok()
}

/// Updates a token's `last_used_at` column value.
///
/// # Arguments
///
/// * `conn` - database connection to use
/// * `token_` - value of the refresh token to update
/// * `last_used_at_` - date value to update column with
///
/// **NOTE**: argument names use trailing underscores as to not conflict with Diesel's imported dsl
/// names.
pub fn update_last_used_at(
    conn: &PgConnection,
    token_: &[u8],
    last_used_at_: chrono::NaiveDateTime,
) -> RbResult<()>
{
    diesel::update(refresh_tokens.filter(token.eq(token_)))
        .set(last_used_at.eq(last_used_at_))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't update last_used_at."))?;

    Ok(())
}
