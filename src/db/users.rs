use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
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

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct NewUser
{
    pub username: String,
    pub password: String,
    pub admin: bool,
}

pub fn all(conn: &PgConnection) -> RbResult<Vec<User>>
{
    users
        .load::<User>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all users."))
}

pub fn find(conn: &PgConnection, user_id: Uuid) -> Option<User>
{
    users.find(user_id).first::<User>(conn).ok()
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> RbResult<()>
{
    let count = diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't create user."))?;

    if count == 0 {
        return Err(RbError::UMDuplicateUser);
    }

    Ok(())
}

pub fn delete(conn: &PgConnection, user_id: Uuid) -> RbResult<()>
{
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't delete user."))?;

    Ok(())
}
