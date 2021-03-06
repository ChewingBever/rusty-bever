use rocket::{serde::json::Json, State};
use serde::Deserialize;

use self::{
    jwt::{generate_jwt_token, JWTResponse},
    pass::verify_user,
};
use crate::{errors::RbResult, guards::User, RbConfig, RbDbConn};

pub mod jwt;
pub mod pass;

#[derive(Deserialize)]
pub struct Credentials
{
    username: String,
    password: String,
}

#[post("/login")]
pub async fn already_logged_in(_user: User) -> String
{
    String::from("You're already logged in!")
}

#[post("/login", data = "<credentials>", rank = 2)]
pub async fn login(
    conn: RbDbConn,
    conf: &State<RbConfig>,
    credentials: Json<Credentials>,
) -> RbResult<Json<JWTResponse>>
{
    let credentials = credentials.into_inner();
    let jwt = conf.jwt.clone();

    // Get the user, if credentials are valid
    let user = conn
        .run(move |c| verify_user(c, &credentials.username, &credentials.password))
        .await?;

    Ok(Json(
        conn.run(move |c| generate_jwt_token(c, &jwt, &user))
            .await?,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest
{
    pub refresh_token: String,
}

#[post("/refresh", data = "<refresh_token_request>")]
pub async fn refresh_token(
    conn: RbDbConn,
    conf: &State<RbConfig>,
    refresh_token_request: Json<RefreshTokenRequest>,
) -> RbResult<Json<JWTResponse>>
{
    let refresh_token = refresh_token_request.into_inner().refresh_token;
    let jwt = conf.jwt.clone();

    Ok(Json(
        conn.run(move |c| crate::auth::jwt::refresh_token(c, &jwt, &refresh_token))
            .await?,
    ))
}
