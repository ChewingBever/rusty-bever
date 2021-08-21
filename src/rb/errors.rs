use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::Status;
use std::io;

pub enum RBError {
    /// When the login requests an unknown user
    UnknownUser,
    /// Invalid login password.
    InvalidPassword,
    /// When a non-admin user tries to use an admin endpoint
    Unauthorized,
    /// When an expired JWT token is used for auth.
    JWTTokenExpired
}

impl<'r> Responder<'r, 'static> for RBError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, message): (Status, &str) = match self {
            UnknownUser => (Status::NotFound, "Unknown user"),
            InvalidPassword => (Status::Unauthorized, "Invalid password"),
            Unauthorized => (Status::Unauthorized, "Unauthorized"),
            JWTTokenExpired => (Status::Unauthorized, "Token expired"),
        };

        let res = Response::new();
        res.set_status(status);
        res.set_sized_body(message.len(), io::Cursor::new(message));

        Ok(res)
    }
}

pub type Result<T> = std::result::Result<T, RBError>;
