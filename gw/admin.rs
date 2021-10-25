use diesel::PgConnection;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    auth::pass::hash_password,
    db,
    errors::{RbError, RbResult},
    guards::Admin,
    RbDbConn,
};

// #[get("/users")]
// pub async fn get_users(_admin: Admin, conn: RbDbConn) -> RbResult<Json<Vec<db::User>>>
// {
//     Ok(Json(conn.run(|c| db::users::all(c)).await?))
// }

#[post("/users", data = "<user>")]
pub async fn create_user(_admin: Admin, conn: RbDbConn, user: Json<db::NewUser>) -> RbResult<()>
{
    Ok(conn
        .run(move |c| db::users::create(c, &user.into_inner()))
        .await?)
}

#[get("/users/<user_id_str>")]
pub async fn get_user_info(
    _admin: Admin,
    conn: RbDbConn,
    user_id_str: &str,
) -> RbResult<Json<db::User>>
{
    let user_id = Uuid::parse_str(user_id_str).map_err(|_| RbError::UMUnknownUser)?;

    match conn.run(move |c| db::users::find(c, user_id)).await {
        Some(user) => Ok(Json(user)),
        None => Err(RbError::UMUnknownUser),
    }
}

pub fn create_admin_user(conn: &PgConnection, username: &str, password: &str) -> RbResult<bool>
{
    let pass_hashed = hash_password(password)?;
    let new_user = db::NewUser {
        username: username.to_string(),
        password: pass_hashed,
        admin: true,
    };

    if db::users::find_by_username(conn, username).is_ok() {
        db::users::create(conn, &new_user);
    }
    // db::users::create_or_update(conn, &new_user)
    //     .map_err(|_| RbError::Custom("Couldn't create admin."))?;

    Ok(true)
}
