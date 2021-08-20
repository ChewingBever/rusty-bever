use diesel::Queryable;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    id: Uuid,
    username: String,
    pub password: String,
    blocked: bool,
    admin: bool,
}
