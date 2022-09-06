pub mod db;
pub mod entities;
pub mod models;
pub mod routes;

use sea_orm_rocket::Database;

use crate::db::pool::Db;
use crate::routes::v1;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![v1::get_manga])
}
