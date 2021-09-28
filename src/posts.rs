use rocket::serde::json::Json;

use crate::{
    db,
    errors::RbResult,
    guards::Admin,
    RbDbConn,
};

#[get("/?<offset>&<limit>")]
pub async fn get(conn: RbDbConn, offset: u32, limit: u32) -> RbResult<Json<Vec<db::Post>>>
{
    Ok(Json(
        conn.run(move |c| db::posts::get(c, offset, limit)).await?,
    ))
}

#[post("/", data = "<new_post>")]
pub async fn create(_admin: Admin, conn: RbDbConn, new_post: Json<db::NewPost>)
    -> RbResult<Json<db::Post>>
{
    Ok(Json(
        conn.run(move |c| db::posts::create(c, &new_post.into_inner()))
            .await?,
    ))
}
