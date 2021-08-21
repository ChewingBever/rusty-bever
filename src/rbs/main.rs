// This needs to be explicitely included before diesel is imported to make sure
// compilation succeeds
extern crate openssl;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel_migrations;

use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

mod auth;

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
    // In debug mode, the admin user is just a test user
    let (admin_user, admin_password): (String, String);

    // if rocket.config().profile == "debug" {
        admin_user = String::from("test");
        admin_password = String::from("test");
    // }else{
    //     admin_user = std::env::var("ADMIN_USER").expect("no admin user provided");
    //     admin_password = std::env::var("ADMIN_PASSWORD").expect("no admin password provided");
    // }
    let conn = RbDbConn::get_one(&rocket)
        .await
        .expect("database connection");
    conn.run(move |c| rb::auth::create_admin_user(c, &admin_user, &admin_password).expect("failed to create admin user")).await;

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
        .attach(AdHoc::try_on_ignite(
                "Create admin user",
                create_admin_user
        ))
        .mount("/auth", auth::routes())
}
