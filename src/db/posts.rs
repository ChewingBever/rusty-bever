//! Handles all posts-related database operations.

use chrono::NaiveDate;
use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{posts, posts::dsl::*},
};

/// Represents a post contained within the database.
#[derive(Queryable)]
pub struct Post
{
    pub id: Uuid,
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
    pub content: String,
}

/// Represents a new post to be added to the database.
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost
{
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
}

/// Returns all posts in the database; should be used with care as this method could quickly return
/// a large amount of data.
///
/// # Arguments
///
/// * `conn` - a reference to a database connection
pub fn all(conn: &PgConnection) -> RbResult<Vec<Post>>
{
    posts
        .load::<Post>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all posts."))
}

/// Insert a new post into the database.
///
/// # Arguments
///
/// * `conn` - reference to a database connection
/// * `new_post` - the new post object to insert
pub fn create(conn: &PgConnection, new_post: &NewPost) -> RbResult<()>
{
    insert_into(posts)
        .values(new_post)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't insert post."))?;

    // TODO check for conflict?

    Ok(())
}
