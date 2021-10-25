use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome, Request},
    State,
};
use sha2::Sha256;

use crate::{auth::jwt::Claims, errors::RbError, RbConfig};

/// Extracts an "Authorization: Bearer" string from the headers.
pub struct Bearer<'a>(&'a str);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Bearer<'r>
{
    type Error = crate::errors::RbError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        // If the header isn't present, just forward to the next route
        let header = match req.headers().get_one("Authorization") {
            None => return Outcome::Forward(()),
            Some(val) => val,
        };

        if header.starts_with("Bearer ") {
            match header.get(7..) {
                Some(s) => Outcome::Success(Self(s)),
                None => Outcome::Failure((Status::Unauthorized, Self::Error::AuthUnauthorized)),
            }
        } else {
            Outcome::Forward(())
        }
    }
}

/// Verifies the provided JWT is valid.
pub struct Jwt(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt
{
    type Error = RbError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        let bearer = try_outcome!(req.guard::<Bearer>().await).0;
        let config = try_outcome!(req.guard::<&State<RbConfig>>().await.map_failure(|_| (
            Status::InternalServerError,
            RbError::Custom("Couldn't get config guard.")
        )));

        let key: Hmac<Sha256> = match Hmac::new_from_slice(&config.jwt.key.as_bytes()) {
            Ok(key) => key,
            Err(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    Self::Error::Custom("Failed to do Hmac thing."),
                ))
            },
        };

        // Verify token using key
        match bearer.verify_with_key(&key) {
            Ok(claims) => Outcome::Success(Self(claims)),
            Err(_) => {
                return Outcome::Failure((Status::Unauthorized, Self::Error::AuthUnauthorized))
            },
        }
    }
}

/// Verifies the JWT has not expired.
pub struct User(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User
{
    type Error = crate::errors::RbError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        let claims = try_outcome!(req.guard::<Jwt>().await).0;

        // Verify key hasn't yet expired
        if chrono::Utc::now().timestamp() > claims.exp {
            Outcome::Failure((Status::Forbidden, Self::Error::AuthTokenExpired))
        } else {
            Outcome::Success(Self(claims))
        }
    }
}

/// Verifies the JWT belongs to an admin.
pub struct Admin(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin
{
    type Error = crate::errors::RbError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>
    {
        let user = try_outcome!(req.guard::<User>().await).0;

        if user.admin {
            Outcome::Success(Self(user))
        } else {
            Outcome::Failure((Status::Unauthorized, RbError::AuthUnauthorized))
        }
    }
}
