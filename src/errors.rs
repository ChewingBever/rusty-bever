use std::io;

use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder, Response},
};

#[derive(Debug)]
pub enum RBError
{
    /// When the login requests an unknown user
    UnknownUser,
    BlockedUser,
    /// Invalid login password.
    InvalidPassword,
    /// When a non-admin user tries to use an admin endpoint
    Unauthorized,
    /// When an expired JWT token is used for auth.
    JWTTokenExpired,
    /// Umbrella error for when something goes wrong whilst creating a JWT token pair
    JWTCreationError,
    JWTError,
    MissingJWTKey,
    PWSaltError,
    AdminCreationError,
    TokenExpired,
    InvalidRefreshToken,
    DuplicateRefreshToken,
    DBError,
    DuplicateUser,
}

impl<'r> Responder<'r, 'static> for RBError
{
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static>
    {
        let (status, message): (Status, &str) = match self {
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
