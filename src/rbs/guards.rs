use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome, Request}
};
use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use rb::auth::Claims;
use sha2::Sha256;

pub struct User(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = rb::errors::RBError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // If the header isn't present, just forward to the next route
        let header = match req.headers().get_one("Authorization") {
            None => return Outcome::Forward(()),
            Some(val) => val,
        };

        if !header.starts_with("Bearer ") {
            return Outcome::Forward(());
        }

        // Extract the jwt token from the header
        let jwt_token = match header.get(7..) {
            Some(token) => token,
            None => return Outcome::Failure((Status::Unauthorized, Self::Error::JWTError)),
        };

        // Get secret & key
        let secret = match std::env::var("JWT_KEY") {
            Ok(key) => key,
            Err(_) => {
                return Outcome::Failure((Status::InternalServerError, Self::Error::MissingJWTKey))
            }
        };
        let key: Hmac<Sha256> = match Hmac::new_from_slice(secret.as_bytes()) {
            Ok(key) => key,
            Err(_) => {
                return Outcome::Failure((Status::InternalServerError, Self::Error::JWTError))
            }
        };

        // Verify token using key
        let claims: Claims = match jwt_token.verify_with_key(&key) {
            Ok(claims) => claims,
            Err(_) => return Outcome::Failure((Status::Unauthorized, Self::Error::Unauthorized)),
        };

        // Verify key hasn't yet expired
        if chrono::Utc::now().timestamp() > claims.exp {
            return Outcome::Failure((Status::Unauthorized, Self::Error::Unauthorized));
        }

        Outcome::Success(Self(claims))
    }
}

pub struct Admin(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = rb::errors::RBError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(req.guard::<User>().await);
        if user.0.admin {
            Outcome::Success(Self(user.0))
        } else {
            Outcome::Forward(())
        }
    }
}
