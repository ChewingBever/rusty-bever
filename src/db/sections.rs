use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{sections, sections::dsl::*},
};

#[derive(Queryable)]
pub struct Section
{
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub has_titles: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "sections"]
// #[serde(rename_all = "camelCase")]
pub struct NewSection
{
    title: String,
    description: Option<String>,
    is_default: Option<bool>,
    has_titles: Option<bool>,
}

pub fn all(conn: &PgConnection) -> RbResult<Vec<Section>>
{
    sections
        .load::<Section>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all sections"))
}

pub fn create(conn: &PgConnection, new_section: &NewSection) -> RbResult<()>
{
    insert_into(sections)
        .values(new_section)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't insert section."))?;

    // TODO check for conflict?

    Ok(())
}
