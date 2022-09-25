pub mod db;
// pub mod models;
pub mod routes;

use db::{AssembleWithOutput, Assemble};
use mangaverse_entity::models::{genre::Genre, pattern::SourcePattern};
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
            let data = Genre::all(dbs)
                .await
                .expect("Error while fetching genres");
            let patterns = SourcePattern::all_with_output(dbs)
                .await
                .expect("Error while fetching patterns");
            rocket
                .manage(std::sync::Arc::new(data))
                .manage(std::sync::Arc::new(patterns))
        }))
        .mount(
            "/public/manga/v1",
            routes![
                v1::get_manga,
                v1::refresh_all,
                v1::get_linked_manga,
                v1::get_all_genres,
                v1::get_chapter,
                v1::get_chapter_position,
                v1::get_manga_from_query,
                v1::get_manga_for_home,
                v1::get_source_patterns,
                v1::insert_manga,
            ],
        )
}
