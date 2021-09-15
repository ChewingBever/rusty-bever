//! Handles refresh token-related database operations.

use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{refresh_tokens, refresh_tokens::dsl::*},
};

/// A refresh token as stored in the database
#[derive(Queryable)]
pub struct RefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub last_used_at: Option<chrono::NaiveDateTime>,
}

/// A new refresh token to be added into the database
#[derive(Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

// TODO add pagination as this could grow very quickly
/// Returns all refresh tokens contained in the database.
///
/// # Arguments
///
/// * `conn` - database connection to use
pub fn all(conn: &PgConnection) -> RbResult<Vec<RefreshToken>>
{
    refresh_tokens
        .load::<RefreshToken>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all refresh tokens."))
}

/// Insert a new refresh token into the database.
///
/// # Arguments
///
/// * `conn` - database connection to use
/// * `new_refresh_token` - token to insert
pub fn create(conn: &PgConnection, new_refresh_token: &NewRefreshToken) -> RbResult<()>
{
    insert_into(refresh_tokens)
        .values(new_refresh_token)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't insert refresh token."))?;

    // TODO check for conflict?

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
    token_val: &[u8],
) -> Option<(RefreshToken, super::users::User)>
{
    // TODO actually check for errors here
    refresh_tokens
        .inner_join(crate::schema::users::dsl::users)
        .filter(token.eq(token_val))
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
