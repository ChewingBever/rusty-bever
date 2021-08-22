use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    errors::RBError,
    schema::{users, users::dsl::*},
};

#[derive(Queryable, Serialize)]
pub struct User
{
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
pub struct NewUser
{
    pub username: String,
    pub password: String,
    pub admin: bool,
}

pub fn all(conn: &PgConnection) -> crate::Result<Vec<User>>
{
    users.load::<User>(conn).map_err(|_| RBError::DBError)
}
