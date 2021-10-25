//! This module handles management of site sections (aka blogs).

use rocket::serde::json::Json;

use crate::{db, errors::RbResult, guards::Admin, RbDbConn};

/// Route for creating a new section.
///
/// # Arguments
///
/// * `_admin` - guard ensuring user is admin
/// * `conn` - guard providing a connection to the database
/// * `new_section` - Json-encoded NewSection object
#[post("/", data = "<new_section>")]
pub async fn create_section(
    _admin: Admin,
    conn: RbDbConn,
    new_section: Json<db::NewSection>,
) -> RbResult<Json<db::Section>>
{
    Ok(Json(
        conn.run(move |c| db::sections::create(c, &new_section.into_inner()))
            .await?,
    ))
}
