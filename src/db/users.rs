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
    pub blocked: bool,
    pub admin: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct NewUser
{
    pub username: String,
    pub password: String,
    pub admin: bool,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "users"]
#[serde(rename_all = "camelCase")]
pub struct PatchSection
{
    username: Option<String>,
    admin: Option<bool>,
}

pub fn get(conn: &PgConnection, offset_: u32, limit_: u32) -> RbResult<Vec<User>>
{
    Ok(users
        .offset(offset_.into())
        .limit(limit_.into())
        .load(conn)
        .map_err(|_| RbError::DbError("Couldn't query users."))?)
}

pub fn find(conn: &PgConnection, user_id: Uuid) -> Option<User>
{
    users.find(user_id).first::<User>(conn).ok()
}

pub fn find_by_username(conn: &PgConnection, username_: &str) -> RbResult<User>
{
    Ok(users
        .filter(username.eq(username_))
        .first::<User>(conn)
        .map_err(|_| RbError::DbError("Couldn't find users by username."))?)
}

/// Insert a new user into the database
///
/// # Arguments
///
/// * `conn` - database connection to use
/// * `new_user` - user to insert
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

/// Either create a new user or update an existing one on conflict.
///
/// # Arguments
///
/// * `conn` - database connection to use
/// * `new_user` - user to insert/update
// pub fn create_or_update(conn: &PgConnection, new_user: &NewUser) -> RbResult<()>
// {
//     diesel::insert_into(users)
//         .values(new_user)
//         .on_conflict(username)
//         .do_update()
//         .set(new_user)
//         .execute(conn)
//         .map_err(|_| RbError::DbError("Couldn't create or update user."))?;

//     Ok(())
// }

/// Delete the user with the given ID.
///
/// # Arguments
///
/// `conn` - database connection to use
/// `user_id` - ID of user to delete
pub fn delete(conn: &PgConnection, user_id: Uuid) -> RbResult<()>
{
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't delete user."))?;

    Ok(())
}

/// Block a user given an ID.
/// In practice, this means updating the user's entry so that the `blocked` column is set to
/// `true`.
///
/// # Arguments
///
/// `conn` - database connection to use
/// `user_id` - ID of user to block
pub fn block(conn: &PgConnection, user_id: Uuid) -> RbResult<()>
{
    diesel::update(users.filter(id.eq(user_id)))
        .set(blocked.eq(true))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't block user."))?;

    Ok(())
}
