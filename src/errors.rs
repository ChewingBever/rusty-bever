use std::io;

use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder, Response},
};

#[derive(Debug)]
pub enum RbError
{
    AuthUnknownUser,
    AuthBlockedUser,
    AuthInvalidPassword,
    AuthUnauthorized,
    AuthTokenExpired,
    AuthRefreshTokenExpired,
    AuthInvalidRefreshToken,
    AuthDuplicateRefreshToken,

    Custom(&'static str),

    AdminCreationError,
    DBError,
    DuplicateUser,
}

impl RbError {
    pub fn status(&self) -> Status {
        Status::NotFound
    }

    pub fn message(&self) -> &'static str {
        match self {

        }
    }
}

impl<'r> Responder<'r, 'static> for RBError
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static>
    {
        let (status, message): (Status, &'static str) = match self {
            RBError::UnknownUser => (Status::NotFound, "Unknown user"),
            RBError::BlockedUser => (Status::Unauthorized, "This user is blocked"),
            RBError::InvalidPassword => (Status::Unauthorized, "Invalid password"),
            RBError::Unauthorized => (Status::Unauthorized, "Unauthorized"),
            RBError::JWTTokenExpired => (Status::Unauthorized, "Token expired"),
            RBError::JWTCreationError | RBError::MissingJWTKey => {
                (Status::InternalServerError, "Failed to create tokens.")
            }
            RBError::InvalidRefreshToken | RBError::DuplicateRefreshToken => {
                (Status::Unauthorized, "Invalid refresh token.")
            }
            RBError::DuplicateUser => (Status::Conflict, "User already exists"),
            _ => (Status::InternalServerError, "Internal server error"),
        };

        let mut res = Response::new();
        res.set_status(status);
        res.set_sized_body(message.len(), io::Cursor::new(message));

        Ok(res)
    }
}

pub type Result<T> = std::result::Result<T, RBError>;
