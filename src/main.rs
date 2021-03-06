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
#[cfg(any(feature = "web", feature = "docs"))]
use rocket::fs;
use rocket::{
    fairing::AdHoc,
    http::Status,
    serde::json::{json, Value},
    Build, Orbit, Request, Rocket,
};
use rocket_sync_db_pools::database;
use serde::{Deserialize, Serialize};

mod admin;
pub mod auth;
pub mod db;
pub mod errors;
pub mod guards;
pub mod posts;
pub(crate) mod schema;
pub mod sections;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[database("postgres_rb")]
pub struct RbDbConn(diesel::PgConnection);

#[catch(default)]
fn default_catcher(status: Status, _: &Request) -> Value
{
    json!({"status": status.code, "message": ""})
}

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

async fn create_admin_user<'a>(rocket: &'a Rocket<Orbit>)
{
    let config = rocket.state::<RbConfig>().expect("RbConfig instance");
    let admin_user = config.admin_user.clone();
    let admin_pass = config.admin_pass.clone();

    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(move |c| {
        admin::create_admin_user(c, &admin_user, &admin_pass).expect("failed to create admin user")
    })
    .await;
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

    // This mut is necessary when the "docs" or "web" feature is enabled, as these further modify
    // the instance variable
    #[allow(unused_mut)]
    let mut instance = rocket::custom(figment)
        .attach(RbDbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Run database migrations",
            run_db_migrations,
        ))
        // .attach(AdHoc::try_on_ignite("Create admin user", create_admin_user))
        .attach(AdHoc::config::<RbConfig>())
        .register("/", catchers![default_catcher])
        .mount(
            "/api/auth",
            routes![auth::already_logged_in, auth::login, auth::refresh_token,],
        )
        .mount(
            "/api/admin",
            routes![admin::create_user, admin::get_user_info],
        )
        .mount("/api/sections", routes![sections::create_section])
        .mount("/api/posts", routes![posts::get, posts::create]);

    // It's weird that this is allowed, but the line on its own isn't
    #[cfg(feature = "web")]
    {
        instance = instance.mount(
            "/",
            fs::FileServer::new(
                "/var/www/html/web",
                fs::Options::Index | fs::Options::NormalizeDirs,
            ),
        );
    }

    #[cfg(feature = "docs")]
    {
        instance = instance.mount(
            "/docs",
            fs::FileServer::new(
                "/var/www/html/docs",
                fs::Options::Index | fs::Options::NormalizeDirs,
            ),
        );
    }

    instance
}
