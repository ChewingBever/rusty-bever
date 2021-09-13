use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
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

#[derive(Insertable)]
#[table_name = "sections"]
pub struct NewSection
{
    title: String,
    description: Option<String>,
    is_default: bool,
    has_titles: bool,
}

pub fn all(conn: &PgConnection) -> RbResult<Vec<Section>>
{
    sections.load::<Section>(conn).map_err(|_| RbError::DbError("Couldn't get all sections"))
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
