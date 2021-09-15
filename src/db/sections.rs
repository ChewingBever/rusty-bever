//! Handles all section-related database operations.

use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{sections, sections::dsl::*},
};

/// Represents a section contained in the database.
#[derive(Queryable)]
pub struct Section
{
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub has_titles: bool,
}

/// A new section to be added into the database.
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

/// Returns all sections in the database.
///
/// # Arguments
///
/// * `conn` - reference to a database connection
pub fn all(conn: &PgConnection) -> RbResult<Vec<Section>>
{
    sections
        .load::<Section>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all sections"))
}

/// Inserts a new section into the database.
///
/// # Arguments
///
/// * `conn` - reference to a database connection
/// * `new_section` - the new section to be added
pub fn create(conn: &PgConnection, new_section: &NewSection) -> RbResult<()>
{
    insert_into(sections)
        .values(new_section)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't insert section."))?;

    // TODO check for conflict?

    Ok(())
}
