use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
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

#[derive(Insertable, AsChangeset, Deserialize)]
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

pub fn find(conn: &PgConnection, user_id: Uuid) -> Option<User>
{
    users.find(user_id).first::<User>(conn).ok()
}

pub fn create(conn: &PgConnection, new_user: &NewUser) -> crate::Result<()>
{
    let count = diesel::insert_into(users)
        .values(new_user)
        .execute(conn)
        .map_err(|_| RBError::DBError)?;

    if count == 0 {
        return Err(RBError::DuplicateUser);
    }

    Ok(())
}

pub fn delete(conn: &PgConnection, user_id: Uuid) -> crate::Result<()>
{
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .map_err(|_| RBError::DBError)?;

    Ok(())
}
