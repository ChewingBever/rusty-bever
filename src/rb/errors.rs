use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io;

#[derive(Debug)]
pub enum RBError {
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
    PWSaltError,
    AdminCreationError,
}

impl<'r> Responder<'r, 'static> for RBError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, message): (Status, &str) = match self {
            RBError::UnknownUser => (Status::NotFound, "Unknown user"),
            RBError::BlockedUser => (Status::Unauthorized, "This user is blocked"),
            RBError::InvalidPassword => (Status::Unauthorized, "Invalid password"),
            RBError::Unauthorized => (Status::Unauthorized, "Unauthorized"),
            RBError::JWTTokenExpired => (Status::Unauthorized, "Token expired"),
            RBError::JWTCreationError => (Status::InternalServerError, "Failed to create tokens."),
            _ => (Status::InternalServerError, "Internal server error")
        };

        let mut res = Response::new();
        res.set_status(status);
        res.set_sized_body(message.len(), io::Cursor::new(message));

        Ok(res)
    }
}

pub type Result<T> = std::result::Result<T, RBError>;
