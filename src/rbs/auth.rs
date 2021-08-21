use crate::guards::User;
use crate::RbDbConn;
use rb::auth::{generate_jwt_token, verify_user, JWTResponse};
use rocket::serde::json::Json;
use serde::Deserialize;

pub(crate) fn routes() -> Vec<rocket::Route> {
    routes![login, already_logged_in, me]
}

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/login")]
async fn already_logged_in(_user: User) -> String {
    String::from("You're already logged in!")
}

#[post("/login", data = "<credentials>", rank = 2)]
async fn login(conn: RbDbConn, credentials: Json<Credentials>) -> rb::Result<Json<JWTResponse>> {
    let credentials = credentials.into_inner();

    // Get the user, if credentials are valid
    let user = conn
        .run(move |c| verify_user(c, &credentials.username, &credentials.password))
        .await?;

    Ok(Json(conn.run(move |c| generate_jwt_token(c, &user)).await?))
}

// #[post("/refresh", data=)]
