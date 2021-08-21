use crate::schema::{refresh_tokens, users};
use diesel::{Insertable, Queryable, AsChangeset};
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub blocked: bool,
    pub admin: bool,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub token: Vec<u8>,
    pub user_id: Uuid,
    pub expires_at: chrono::NaiveDateTime
}
