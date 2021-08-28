use rb::{db, errors::RbError};
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{guards::Admin, RbDbConn};

pub fn routes() -> Vec<rocket::Route>
{
    routes![get_users, get_user_info, create_user]
}

#[get("/users")]
async fn get_users(admin: Admin, conn: RbDbConn) -> rb::Result<Json<Vec<db::User>>>
{
    Ok(Json(conn.run(|c| db::users::all(c)).await?))
}

#[post("/users", data = "<user>")]
async fn create_user(admin: Admin, conn: RbDbConn, user: Json<db::NewUser>) -> rb::Result<()>
{
    Ok(conn
        .run(move |c| db::users::create(c, &user.into_inner()))
        .await?)
}

#[get("/users/<user_id_str>")]
async fn get_user_info(
    _admin: Admin,
    conn: RbDbConn,
    user_id_str: &str,
) -> rb::Result<Json<db::User>>
{
    let user_id = Uuid::parse_str(user_id_str).map_err(|_| RbError::UMUnknownUser)?;

    match conn.run(move |c| db::users::find(c, user_id)).await {
        Some(user) => Ok(Json(user)),
        None => Err(RbError::UMUnknownUser),
    }
}
