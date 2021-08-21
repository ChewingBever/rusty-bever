use crate::RbDbConn;
use rb::auth::verify_user;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[post("/login", data = "<credentials>")]
async fn login(conn: RbDbConn, credentials: Json<Credentials>) {
    let credentials = credentials.into_inner();

    let user = conn
        .run(move |c| verify_user(c, &credentials.username, &credentials.password))
        .await;
    user
}

// /refresh
// /logout
