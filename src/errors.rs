use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder},
    serde::json::json,
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
    AuthMissingHeader,

    // UM = User Management
    UMDuplicateUser,
    UMUnknownUser,

    DbError(&'static str),
    Custom(&'static str),
}

impl RbError
{
    pub fn status(&self) -> Status
    {
        // Every entry gets its own line for easy editing later when needed
        match self {
            RbError::AuthUnknownUser => Status::NotFound,
            RbError::AuthBlockedUser => Status::Forbidden,
            RbError::AuthInvalidPassword => Status::Unauthorized,
            RbError::AuthUnauthorized => Status::Unauthorized,
            RbError::AuthTokenExpired => Status::Unauthorized,
            RbError::AuthRefreshTokenExpired => Status::Unauthorized,
            RbError::AuthInvalidRefreshToken => Status::Unauthorized,
            RbError::AuthDuplicateRefreshToken => Status::Unauthorized,
            RbError::AuthMissingHeader => Status::BadRequest,

            RbError::UMDuplicateUser => Status::Conflict,

            RbError::Custom(_) => Status::InternalServerError,
            _ => Status::InternalServerError,
        }
    }

    pub fn message(&self) -> &'static str
    {
        match self {
            RbError::AuthUnknownUser => "This user doesn't exist.",
            RbError::AuthBlockedUser => "This user is blocked.",
            RbError::AuthInvalidPassword => "Invalid credentials.",
            RbError::AuthUnauthorized => "You are not authorized to access this resource.",
            RbError::AuthTokenExpired => "This token is not valid anymore.",
            RbError::AuthRefreshTokenExpired => "This refresh token is not valid anymore.",
            RbError::AuthInvalidRefreshToken => "This refresh token is not valid.",
            RbError::AuthDuplicateRefreshToken => {
                "This refresh token has already been used. The user has been blocked."
            },
            RbError::AuthMissingHeader => "Missing Authorization header.",

            RbError::UMDuplicateUser => "This user already exists.",

            RbError::Custom(message) => message,
            _ => "",
        }
    }
}

impl<'r> Responder<'r, 'static> for RbError
{
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static>
    {
        let status = self.status();
        let content = json!({
            "status": status.code,
            "message": self.message(),
        });

        // TODO add status to response
        content.respond_to(req)
    }
}

/// Type alias for results that can return an RbError
pub type RbResult<T> = std::result::Result<T, RbError>;

/// Type alias for optional results that can fail & return an RbError
pub type RbOption<T> = RbResult<Option<T>>;
