pub mod models;
pub mod routes;

use rocket_db_pools::{sqlx, Database};

use crate::routes::v1;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("manga_server")]
pub struct Db(sqlx::MySqlPool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![v1::get_manga])
}
