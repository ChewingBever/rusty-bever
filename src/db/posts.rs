use chrono::NaiveDate;
use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{posts, posts::dsl::*},
};

#[derive(Queryable, Serialize)]
pub struct Post
{
    pub id: Uuid,
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
    pub content: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "posts"]
#[serde(rename_all = "camelCase")]
pub struct NewPost
{
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
    pub content: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct PatchPost
{
    pub section_id: Option<Uuid>,
    pub title: Option<String>,
    pub publish_date: Option<NaiveDate>,
    pub content: Option<String>,
}

pub fn get(conn: &PgConnection, offset_: u32, limit_: u32) -> RbResult<Vec<Post>>
{
    Ok(posts
        .offset(offset_.into())
        .limit(limit_.into())
        .load::<Post>(conn)
        .map_err(|_| RbError::DbError("Couldn't query posts."))?)
}

pub fn create(conn: &PgConnection, new_post: &NewPost) -> RbResult<Post>
{
    Ok(insert_into(posts)
        .values(new_post)
        .get_result::<Post>(conn)
        .map_err(|_| RbError::DbError("Couldn't insert post."))?)

    // TODO check for conflict?
}

pub fn update(conn: &PgConnection, post_id: &Uuid, patch_post: &PatchPost) -> RbResult<Post>
{
    Ok(diesel::update(posts.filter(id.eq(post_id)))
        .set(patch_post)
        .get_result::<Post>(conn)
        .map_err(|_| RbError::DbError("Couldn't update post."))?)
}

pub fn delete(conn: &PgConnection, post_id: &Uuid) -> RbResult<()>
{
    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't delete post."))?;

    Ok(())
}
