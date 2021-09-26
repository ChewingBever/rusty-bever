use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{sections, sections::dsl::*},
};

#[derive(Queryable, Serialize)]
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
#[serde(rename_all = "camelCase")]
pub struct NewSection
{
    title: String,
    description: Option<String>,
    is_default: Option<bool>,
    has_titles: Option<bool>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "sections"]
#[serde(rename_all = "camelCase")]
pub struct PatchSection
{
    title: Option<String>,
    description: Option<String>,
    is_default: Option<bool>,
    has_titles: Option<bool>,
}

pub fn get(conn: &PgConnection, offset_: u32, limit_: u32) -> RbResult<Vec<Section>>
{
    Ok(sections
        .offset(offset_.into())
        .limit(limit_.into())
        .load::<Section>(conn)
        .map_err(|_| RbError::DbError("Couldn't query sections."))?)
}

pub fn create(conn: &PgConnection, new_post: &NewSection) -> RbResult<Section>
{
    Ok(insert_into(sections)
        .values(new_post)
        .get_result::<Section>(conn)
        .map_err(|_| RbError::DbError("Couldn't insert section."))?)

    // TODO check for conflict?
}

pub fn update(conn: &PgConnection, post_id: &Uuid, patch_post: &PatchSection) -> RbResult<Section>
{
    Ok(diesel::update(sections.filter(id.eq(post_id)))
        .set(patch_post)
        .get_result::<Section>(conn)
        .map_err(|_| RbError::DbError("Couldn't update section."))?)
}

pub fn delete(conn: &PgConnection, post_id: &Uuid) -> RbResult<()>
{
    diesel::delete(sections.filter(id.eq(post_id)))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't delete section."))?;

    Ok(())
}
