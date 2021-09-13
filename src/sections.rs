use rocket::serde::json::Json;

use crate::{db, errors::RbResult, guards::Admin, RbDbConn};

/// Create a new section.
#[post("/", data = "<new_section>")]
pub async fn create_section(
    _admin: Admin,
    conn: RbDbConn,
    new_section: Json<db::NewSection>,
) -> RbResult<()>
{
    Ok(conn
        .run(move |c| db::sections::create(c, &new_section.into_inner()))
        .await?)
}
