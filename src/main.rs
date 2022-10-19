pub mod db;
pub mod routes;

use std::collections::HashMap;

use db::{Assemble, AssembleWithOutput};
use mangaverse_entity::models::pattern::SourcePattern;
use mangaverse_entity::models::{genre::Genre, source::SourceTable};
use mangaverse_sources::Context;
use rocket::fairing::AdHoc;
use rocket_db_pools::{sqlx, Database};

use crate::routes::v1;
use crate::routes::v2;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("manga_server")]
pub struct Db(sqlx::MySqlPool);

pub struct AllPatterns {
    patterns: HashMap<String, String>
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::on_ignite("populate state", |rocket| async {
            let dbs = Db::fetch(&rocket).expect("No db");
            let data = Genre::all(dbs).await.expect("Error while fetching genres");
            let sources = SourceTable::all_with_output(dbs).await.expect("Error while fetching sources");
            let only_sources = sources.iter().map(|(k, v)| (v.name.clone(), k.clone())).collect::<HashMap<_, _>>();
            let all_patterns = AllPatterns { patterns:  SourcePattern::all_with_output(dbs).await.expect("Error while fetching patterns") };
            let context = Context {
                genres: data
                    .iter()
                    .map(|f| (f.name.clone(), f.clone()))
                    .collect::<HashMap<String, Genre>>(),
                sources
            };
            rocket
                .manage(std::sync::Arc::new(data))
                .manage(std::sync::Arc::new(context))
                .manage(std::sync::Arc::new(only_sources))
                .manage(std::sync::Arc::new(all_patterns))
        }))
        .mount(
            "/public/manga/v1",
            routes![
                v1::get_manga,
                v1::refresh_all,
                v1::refresh_one,
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
        .mount(
            "/public/manga/v2",
            routes![
                v2::get_sources
            ]
        )
}
