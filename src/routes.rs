#[derive(Responder, Debug)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    pub message: String,
}

impl From<sqlx::Error> for ErrorResponder {
    fn from(a: sqlx::Error) -> Self {
        ErrorResponder {
            message: a.to_string(),
        }
    }
}

impl From<mangaverse_sources::MSError> for ErrorResponder {
    fn from(a: mangaverse_sources::MSError) -> Self {
        ErrorResponder { message: a.message }
    }
}

impl From<std::io::Error> for ErrorResponder {
    fn from(a: std::io::Error) -> Self {
        ErrorResponder {
            message: a.to_string(),
        }
    }
}

pub mod v1 {

    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::db::manga::{get_urls_from_linked_ids, check_if_manga_exists};
    use crate::{Db, AllPatterns};
    use crate::{
        db::{Assemble, AssembleWithArgs, AssembleWithArgsAndOutput},
        routes::ErrorResponder,
    };
    use chrono::Utc;
    use mangaverse_entity::models::{
        genre::Genre,
        manga::{CompleteManga, LinkedManga},
        page::{ChapterPosition, PageURL},
        query::{MangaQuery, MangaQueryResponse, MangaRequest},
    };
    use mangaverse_sources::db::manga::{get_manga_from_id as get_db_manga_id, insert_manga as insert_manga_db};
    use mangaverse_sources::db::manga::get_manga_from_url as get_db_manga_url;
    use mangaverse_sources::db::manga::update_manga;
    use mangaverse_sources::manganelo::entity::get_manga as get_manganelo_manga;
    use mangaverse_sources::readm::entity::get_manga as get_readm_manga;
    use mangaverse_sources::Context;
    use rocket::serde::json::Json;
    use rocket::serde::uuid::Uuid;
    use rocket::State;
    use rocket_db_pools::Connection;
    use sqlx::pool::PoolConnection;
    use sqlx::MySql;

    use mangaverse_sources::Result as MSResult;

    pub async fn update_request_from_url(
        context: &Context,
        url: &str,
        conn: &mut PoolConnection<MySql>
    ) -> MSResult<()> {
        println!("Processing {}", url);

        let stored = get_db_manga_url(url, conn, context).await?;

        if let Some(u) = stored.last_watch_time {
            if Utc::now().timestamp_millis() - u <= 15 * 60 * 1000 {
                println!("Not Watching because of time limit");
                return Ok(());
            }
        }

        let mut t = match stored.source {
            x if x.name == "manganelo" => {
                get_manganelo_manga(url.to_owned(), x, &context.genres).await?
            }
            x if x.name == "readm" => get_readm_manga(url.to_owned(), x, &context.genres).await?,
            _ => return Ok(()),
        };

        update_manga(&stored, &mut t, conn).await?;

        println!("Finished Processing {}", url);

        Ok(())
    }

    #[allow(clippy::identity_op)]
    pub async fn update_request_from_id(
        context: &Context,
        id: &str,
        conn: &mut PoolConnection<MySql>
    ) -> MSResult<()> {
        println!("Processing {}", id);

        let stored = get_db_manga_id(id, conn, context).await?;

        if let Some(u) = stored.last_watch_time {
            if Utc::now().timestamp_millis() - u <= 1 * 60 * 1000 {
                println!("Not Watching because of time limit");
                return Ok(());
            }
        }

        let mut t = match stored.source {
            x if x.name == "manganelo" => {
                get_manganelo_manga(stored.url.to_owned(), x, &context.genres).await?
            }
            x if x.name == "readm" => {
                get_readm_manga(stored.url.to_owned(), x, &context.genres).await?
            }
            _ => return Ok(()),
        };

        update_manga(&stored, &mut t, conn).await?;

        println!("Finished Processing {}", id);

        Ok(())
    }

    #[get("/<id>")]
    pub async fn get_manga(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<CompleteManga>, ErrorResponder> {
        Ok(Json(
            CompleteManga::assemble(id.to_string().as_str(), &mut conn).await?,
        ))
    }

    #[post("/refresh", data = "<ids>")]
    pub async fn refresh_all(
        ids: Json<Vec<Uuid>>,
        mut conn: Connection<Db>,
        context: &State<Arc<Context>>,
    ) -> Result<(), ErrorResponder> {
        println!("Processing Refresh Request");

        let all = get_urls_from_linked_ids(
            ids.iter()
                .map(|f| f.to_string())
                .collect::<Vec<_>>()
                .as_slice(),
            &mut conn,
        )
        .await?;

        for t in all {
            if let Err(e) = update_request_from_url(context, t.as_str(), &mut *conn).await {
                println!("{:#?}", e);
            }
        }

        println!("Processed Refresh Request");

        Ok(())
    }

    #[post("/refreshOne", data = "<ids>")]
    pub async fn refresh_one(
        ids: Json<Vec<String>>,
        mut conn: Connection<Db>,
        context: &State<Arc<Context>>,
    ) -> Result<(), ErrorResponder> {
        println!("Processing Refresh Request");

        for t in ids.iter() {
            if let Err(e) = update_request_from_url(context, t.as_str(), &mut *conn).await {
                println!("{:#?}", e);
            }
        }

        println!("Processed Refresh Request");

        Ok(())
    }

    #[get("/part/<id>")]
    pub async fn get_linked_manga(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<LinkedManga>, ErrorResponder> {
        Ok(Json(
            LinkedManga::assemble(&id.to_string(), &mut conn).await?,
        ))
    }

    #[get("/genres")]
    pub async fn get_all_genres(
        genres: &State<Arc<Vec<Genre>>>,
    ) -> Result<Json<&[Genre]>, ErrorResponder> {
        Ok(Json(genres.as_slice()))
    }

    #[get("/chapter/<id>")]
    pub async fn get_chapter(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<Vec<PageURL>>, ErrorResponder> {
        Ok(Json(
            PageURL::assemble_many(&id.to_string(), &mut conn).await?,
        ))
    }

    #[get("/chapter/position/<manga_id>/<sequence_number>")]
    pub async fn get_chapter_position(
        mut conn: Connection<Db>,
        manga_id: Uuid,
        sequence_number: u32,
    ) -> Result<Json<ChapterPosition>, ErrorResponder> {
        Ok(Json(
            ChapterPosition::assemble_with_args(&manga_id.to_string(), sequence_number, &mut conn)
                .await?,
        ))
    }

    #[post("/search", data = "<query>")]
    pub async fn get_manga_from_query(
        query: Json<MangaQuery>,
        mut conn: Connection<Db>,
    ) -> Result<Json<MangaQueryResponse>, ErrorResponder> {
        Ok(Json(
            MangaQueryResponse::assemble_with_args("", query.0, &mut conn).await?,
        ))
    }

    #[post("/home", data = "<query>")]
    pub async fn get_manga_for_home(
        query: Json<MangaQuery>,
        mut conn: Connection<Db>,
    ) -> Result<Json<MangaQueryResponse>, ErrorResponder> {
        Ok(Json(
            MangaQueryResponse::all_with_args_and_output(query.0, &mut conn).await?,
        ))
    }

    #[get("/currentSources")]
    pub async fn get_source_patterns(
        patterns: &State<Arc<AllPatterns>>,
    ) -> Result<Json<&HashMap<String, String>>, ErrorResponder> {
        Ok(Json(&patterns.patterns))
    }

    #[post("/insert", data = "<req>")]
    pub async fn insert_manga(
        context: &State<Arc<Context>>,
        req: Json<MangaRequest>,
        mut conn: Connection<Db>
    ) -> Result<(), ErrorResponder> {
        // WIP

        let mut t = match context.sources.get(req.0.id.as_str()) {
            Some(x) if x.name == "manganelo" => {
                get_manganelo_manga(req.url.to_string(), x, &context.genres).await?
            }
            Some(x) if x.name == "readm" => {
                get_readm_manga(req.url.to_string(), x, &context.genres).await?
            }
            _ => return Ok(()),
        };

        if check_if_manga_exists(req.url.as_str(), &mut conn).await? {
            update_request_from_url(context, req.url.as_str(), &mut *conn).await?;
        } else {
            insert_manga_db(&mut t, &mut *conn).await?;
        }

        Ok(())
    }
}

pub mod v2 {
    use std::{sync::Arc, collections::HashMap};
    use rocket::{State, serde::json::Json};
    use crate::routes::ErrorResponder;


    #[get("/allSources")]
    pub async fn get_sources(
        sources: &State<Arc<HashMap<String, String>>>,
    ) -> Result<Json<&HashMap<String, String>>, ErrorResponder> {
        Ok(Json(sources))
    }

}