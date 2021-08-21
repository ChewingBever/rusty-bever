use crate::RbDbConn;
use rb::auth::{generate_jwt_token, verify_user, JWTResponse};
use rocket::serde::json::Json;
use serde::Deserialize;
use crate::guards::User;

pub(crate) fn routes() -> Vec<rocket::Route> {
    routes![login, me]
}


#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

// TODO add catch for when user immediately requests new JWT token (they could totally spam this)

#[post("/login", data = "<credentials>")]
async fn login(conn: RbDbConn, credentials: Json<Credentials>) -> rb::Result<Json<JWTResponse>> {
    let credentials = credentials.into_inner();

    // Get the user, if credentials are valid
    let user = conn
        .run(move |c| verify_user(c, &credentials.username, &credentials.password))
        .await?;

    Ok(Json(conn.run(move |c| generate_jwt_token(c, &user)).await?))
}

#[get("/me")]
async fn me(claims: User) -> String {
    String::from("You are logged in!")
}

// /refresh
// /logout
