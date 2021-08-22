use rb::db::users::{User, NewUser};
use rb::db::users as db_users;
use rb::errors::RBError;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::{guards::Admin, RbDbConn};

pub fn routes() -> Vec<rocket::Route>
{
    routes![get_users, get_user_info]
}

#[get("/users")]
async fn get_users(admin: Admin, conn: RbDbConn) -> rb::Result<Json<Vec<User>>>
{
    Ok(Json(conn.run(|c| rb::db::users::all(c)).await?))
}

#[post("/users", data="<user>")]
async fn create_user(admin: Admin, conn: RbDbConn, user: Json<NewUser>) -> rb::Result<()> {
    Ok(conn.run(move |c| db_users::create(c, &user.into_inner())).await?)
}

#[get("/users/<user_id_str>")]
async fn get_user_info(_admin: Admin, conn: RbDbConn, user_id_str: String) -> rb::Result<Json<User>> {
    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| RBError::UnknownUser)?;

    match conn.run(move |c| db_users::find(c, user_id)).await {
        Some(user) => Ok(Json(user)),
        None => Err(RBError::UnknownUser),
    }
}
