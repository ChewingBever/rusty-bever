use crate::errors::RBError;
use crate::models::User;
use diesel::prelude::*;
use crate::schema::users::dsl as users;

pub fn get_users(conn: &PgConnection) -> crate::Result<Vec<User>> {
    users::users.load::<User>(conn).map_err(|_| RBError::DBError)
}
