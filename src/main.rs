pub mod models;
pub mod routes;

use models::genre::MangaGenre;
use rocket::fairing::AdHoc;
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
        .attach(AdHoc::on_ignite("populate state", |rocket| async {
            let dbs = Db::fetch(&rocket).expect("No db");
            let data = MangaGenre::all(dbs).await.expect("Should have worked");
            rocket.manage(std::sync::Arc::new(data))
        }))
        .mount("/", routes![v1::get_manga])
}
