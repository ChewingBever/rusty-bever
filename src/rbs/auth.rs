use crate::RbDbConn;
use serde::Deserialize;
use rocket::serde::json::Json;

#[derive(Deserialize)]
struct Credentials {
    username: String,
    password: String
}

#[post("/login", data="<credentials>")]
async fn login(conn: RbDbConn, credentials: Json<Credentials>) {

}

// /refresh
// /logout
