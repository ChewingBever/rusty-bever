use crate::schema::users;
use diesel::{AsChangeset, Insertable, Queryable, prelude::*};
use serde::Serialize;
use uuid::Uuid;
use crate::schema::users::dsl::*;
use crate::errors::RBError;

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

pub fn all(conn: &PgConnection) -> crate::Result<Vec<User>> {
    users.load::<User>(conn).map_err(|_| RBError::DBError)
}
