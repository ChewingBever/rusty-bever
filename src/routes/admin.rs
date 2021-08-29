use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{
    db,
    errors::{RbError, RbResult},
    guards::Admin,
    RbDbConn,
};

pub fn routes() -> Vec<rocket::Route>
{
    routes![get_users, get_user_info, create_user]
}

#[get("/users")]
async fn get_users(_admin: Admin, conn: RbDbConn) -> RbResult<Json<Vec<db::User>>>
{
    Ok(Json(conn.run(|c| db::users::all(c)).await?))
}

#[post("/users", data = "<user>")]
async fn create_user(_admin: Admin, conn: RbDbConn, user: Json<db::NewUser>) -> RbResult<()>
{
    Ok(conn
        .run(move |c| db::users::create(c, &user.into_inner()))
        .await?)
}

#[get("/users/<user_id_str>")]
async fn get_user_info(_admin: Admin, conn: RbDbConn, user_id_str: &str)
    -> RbResult<Json<db::User>>
{
    let user_id = Uuid::parse_str(user_id_str).map_err(|_| RbError::UMUnknownUser)?;

    match conn.run(move |c| db::users::find(c, user_id)).await {
        Some(user) => Ok(Json(user)),
        None => Err(RbError::UMUnknownUser),
    }
}
