use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{refresh_tokens, refresh_tokens::dsl::*},
};

#[derive(Queryable)]
pub struct RefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
    pub last_used_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken
{
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime,
}

pub fn all(conn: &PgConnection) -> RbResult<Vec<RefreshToken>>
{
    refresh_tokens
        .load::<RefreshToken>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all refresh tokens."))
}

pub fn create(conn: &PgConnection, new_refresh_token: &NewRefreshToken) -> RbResult<()>
{
    insert_into(refresh_tokens)
        .values(new_refresh_token)
        .execute(conn)
        .map_err(|_| RbError::Custom("Couldn't insert refresh token."))?;

    // TODO check for conflict?

    Ok(())
}

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
        .map_err(|_| RbError::Custom("Couldn't get refresh token & user."))
        .ok()
}
