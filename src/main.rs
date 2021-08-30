// This needs to be explicitely included before diesel is imported to make sure
// compilation succeeds in the release Docker image.
extern crate openssl;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

mod admin;
pub mod auth;
pub mod db;
pub mod errors;
pub mod guards;
pub(crate) mod schema;

#[database("postgres_rb")]
pub struct RbDbConn(diesel::PgConnection);

embed_migrations!();

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>>
{
    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| match embedded_migrations::run(c) {
        Ok(()) => Ok(rocket),
        Err(_) => Err(rocket),
    })
    .await
}

async fn create_admin_user(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>>
{
    let admin_user = std::env::var("ADMIN_USER").unwrap_or(String::from("admin"));
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or(String::from("password"));

    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(move |c| {
        admin::create_admin_user(c, &admin_user, &admin_password)
            .expect("failed to create admin user")
    })
    .await;

    Ok(rocket)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RbJwtConf
{
    key: String,
    refresh_token_size: usize,
    refresh_token_expire: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RbConfig
{
    admin_user: String,
    admin_pass: String,
    jwt: RbJwtConf,
}

#[launch]
fn rocket() -> _
{
    let figment = Figment::from(rocket::config::Config::default())
        .merge(Yaml::file("Rb.yaml").nested())
        .merge(Env::prefixed("RB_").global());

    rocket::custom(figment)
        .attach(RbDbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Run database migrations",
            run_db_migrations,
        ))
        .attach(AdHoc::try_on_ignite("Create admin user", create_admin_user))
        .attach(AdHoc::config::<RbConfig>())
        .mount(
            "/api/auth",
            routes![auth::already_logged_in, auth::login, auth::refresh_token,],
        )
        .mount(
            "/api/admin",
            routes![admin::get_users, admin::create_user, admin::get_user_info],
        )
}
