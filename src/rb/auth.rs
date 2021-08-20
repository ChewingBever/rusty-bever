use crate::errors::AuthError;
use crate::models::User;
use crate::schema::users::dsl as users;
use argon2::verify_encoded;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn verify_user(conn: &PgConnection, username: &str, password: &str) -> Result<User, AuthError> {
    // TODO handle non-"NotFound" Diesel errors accordingely
    let user = match users::users
        .filter(users::username.eq(username))
        .first::<User>(conn)
    {
        Err(_) => return Err(AuthError::UnknownUser),
        Ok(user) => user,
    };

    match verify_encoded(user.password.as_str(), password.as_bytes()) {
        Ok(true) => Ok(user),
        _ => Err(AuthError::InvalidPassword),
    }
}
