use rb::auth::{
    jwt::{generate_jwt_token, JWTResponse},
    verify_user,
};
use rocket::serde::json::Json;
use serde::Deserialize;

use crate::{guards::User, RbDbConn};

pub(crate) fn routes() -> Vec<rocket::Route>
{
    routes![login, already_logged_in, refresh_token]
}

#[derive(Deserialize)]
struct Credentials
{
    username: String,
    password: String,
}

#[post("/login")]
async fn already_logged_in(_user: User) -> String
{
    String::from("You're already logged in!")
}

#[post("/login", data = "<credentials>", rank = 2)]
async fn login(conn: RbDbConn, credentials: Json<Credentials>) -> rb::Result<Json<JWTResponse>>
{
    let credentials = credentials.into_inner();

    // Get the user, if credentials are valid
    let user = conn
        .run(move |c| verify_user(c, &credentials.username, &credentials.password))
        .await?;

    Ok(Json(conn.run(move |c| generate_jwt_token(c, &user)).await?))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RefreshTokenRequest
{
    pub refresh_token: String,
}

#[post("/refresh", data = "<refresh_token_request>")]
async fn refresh_token(
    conn: RbDbConn,
    refresh_token_request: Json<RefreshTokenRequest>,
) -> rb::Result<Json<JWTResponse>>
{
    let refresh_token = refresh_token_request.into_inner().refresh_token;

    Ok(Json(
        conn.run(move |c| rb::auth::jwt::refresh_token(c, &refresh_token))
            .await?,
    ))
}
