use diesel::{Insertable, Queryable};
use uuid::Uuid;

use crate::schema::refresh_tokens;

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
