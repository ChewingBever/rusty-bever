use diesel::{Queryable, Insertable};
use uuid::Uuid;
use serde::Serialize;
use crate::schema::refresh_tokens;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    blocked: bool,
    pub admin: bool,
}


#[derive(Insertable)]
#[table_name = "refresh_tokens"]
pub struct NewRefreshToken {
    pub token: Vec<u8>,
    pub user_id: Uuid
}
