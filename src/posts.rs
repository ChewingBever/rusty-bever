use rocket::serde::json::Json;

use crate::{db, errors::RbResult, guards::Admin, RbDbConn};

#[get("/?<offset>&<limit>")]
pub async fn get(conn: RbDbConn, offset: u32, limit: u32) -> RbResult<Json<Vec<db::Post>>>
{
    Ok(Json(
        conn.run(move |c| db::posts::get(c, offset, limit)).await?,
    ))
}

#[post("/", data = "<new_post>")]
pub async fn create(
    _admin: Admin,
    conn: RbDbConn,
    new_post: Json<db::NewPost>,
) -> RbResult<Json<db::Post>>
{
    Ok(Json(
        conn.run(move |c| db::posts::create(c, &new_post.into_inner()))
            .await?,
    ))
}

#[patch("/<id>", data = "<patch_post>")]
pub async fn patch(
    _admin: Admin,
    conn: RbDbConn,
    id: uuid::Uuid,
    patch_post: Json<db::PatchPost>,
) -> RbResult<Json<db::Post>>
{
    Ok(Json(
        conn.run(move |c| db::posts::update(c, &id, &patch_post.into_inner()))
            .await?,
    ))
}

#[delete("/<id>")]
pub async fn delete(_admin: Admin, conn: RbDbConn, id: uuid::Uuid) -> RbResult<()>
{
    Ok(conn.run(move |c| db::posts::delete(c, &id)).await?)
}
