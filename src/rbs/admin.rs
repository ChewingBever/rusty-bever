use crate::guards::Admin;
use crate::RbDbConn;
use rb::models::User;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users]
}

#[get("/users")]
async fn get_users(admin: Admin, conn: RbDbConn) -> rb::Result<Json<Vec<User>>> {
    Ok(Json(conn.run(|c| rb::admin::get_users(c)).await?))
}
