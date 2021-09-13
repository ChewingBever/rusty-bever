use chrono::NaiveDate;
use diesel::{insert_into, prelude::*, Insertable, PgConnection, Queryable};
use uuid::Uuid;

use crate::{
    errors::{RbError, RbResult},
    schema::{posts, posts::dsl::*},
};

#[derive(Queryable)]
pub struct Post
{
    pub id: Uuid,
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost
{
    pub section_id: Uuid,
    pub title: Option<String>,
    pub publish_date: NaiveDate,
}

pub fn all(conn: &PgConnection) -> RbResult<Vec<Post>>
{
    posts
        .load::<Post>(conn)
        .map_err(|_| RbError::DbError("Couldn't get all posts."))
}

pub fn create(conn: &PgConnection, new_post: &NewPost) -> RbResult<()>
{
    insert_into(posts)
        .values(new_post)
        .execute(conn)
        .map_err(|_| RbError::DbError("Couldn't insert post."))?;

    // TODO check for conflict?

    Ok(())
}
