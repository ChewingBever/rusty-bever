use rb::db::users::User;
use rocket::serde::json::Json;

use crate::{guards::Admin, RbDbConn};

pub fn routes() -> Vec<rocket::Route>
{
    routes![get_users]
}

#[get("/users")]
async fn get_users(admin: Admin, conn: RbDbConn) -> rb::Result<Json<Vec<User>>>
{
    Ok(Json(conn.run(|c| rb::db::users::all(c)).await?))
}
