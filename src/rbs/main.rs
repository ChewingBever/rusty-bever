// This needs to be explicitely included before diesel is imported to make sure
// compilation succeeds
extern crate openssl;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel_migrations;

use rocket::{fairing::AdHoc, Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RbDbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Run database migrations",
            run_db_migrations,
        ))
}
