// This needs to be explicitely included before diesel is imported to make sure
// compilation succeeds in the release Docker image.
extern crate openssl;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

mod auth;
pub(crate) mod guards;

embed_migrations!();

#[database("postgres_rb")]
pub struct RbDbConn(diesel::PgConnection);

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(|c| match embedded_migrations::run(c) {
        Ok(()) => Ok(rocket),
        Err(_) => Err(rocket),
    })
    .await
}

async fn create_admin_user(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let admin_user = std::env::var("ADMIN_USER").unwrap_or(String::from("admin"));
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or(String::from("password"));

    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(move |c| {
        rb::auth::create_admin_user(c, &admin_user, &admin_password)
            .expect("failed to create admin user")
    })
    .await;

    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RbDbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Run database migrations",
            run_db_migrations,
        ))
        .attach(AdHoc::try_on_ignite("Create admin user", create_admin_user))
        .mount("/api/auth", auth::routes())
}
