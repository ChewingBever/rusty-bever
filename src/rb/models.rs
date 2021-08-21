use diesel::Queryable;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    blocked: bool,
    admin: bool,
}
