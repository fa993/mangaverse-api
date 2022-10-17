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

impl From<mangaverse_sources::Error> for ErrorResponder {
    fn from(a: mangaverse_sources::Error) -> Self {
        ErrorResponder {
            message: a.to_string(),
        }
    }
}
pub mod v1 {

    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::Db;
    use crate::{
        db::{Assemble, AssembleWithArgs, AssembleWithArgsAndOutput},
        routes::ErrorResponder,
    };
    use mangaverse_entity::models::{
        genre::Genre,
        manga::{CompleteManga, LinkedManga},
        page::{ChapterPosition, PageURL},
        query::{MangaQuery, MangaQueryResponse, MangaRequest},
    };
    use mangaverse_sources::db::manga::update_manga;
    use mangaverse_sources::manganelo::entity::get_manga as get_manganelo_manga;
    use mangaverse_sources::readm::entity::get_manga as get_readm_manga;
    use mangaverse_sources::Context;
    use rocket::serde::json::Json;
    use rocket::serde::uuid::Uuid;
    use rocket::tokio::task;
    use rocket::State;
    use rocket_db_pools::Connection;

    pub async fn update_request(
        context: &Context,
        req: &MangaRequest,
        pool: &Db,
    ) -> Result<(), mangaverse_sources::Error> {
        match context.sources.get(req.id.as_str()) {
            Some(x) if x.name == "manganelo" => {
                let mut t = get_manganelo_manga(req.url.clone(), x, &context.genres).await?;
                update_manga(&req.url, &mut t, &pool.0, context).await?;
            }
            Some(x) if x.name == "readm" => {
                let mut t = get_readm_manga(req.url.clone(), x, &context.genres).await?;
                update_manga(&req.url, &mut t, &pool.0, context).await?;
            }
            _ => {}
        };
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

    #[post("/refresh", data = "<_ids>")]
    pub fn refresh_all(_ids: Json<Vec<Uuid>>) -> Result<(), ErrorResponder> {
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
        patterns: &State<Arc<HashMap<String, String>>>,
    ) -> Result<Json<&HashMap<String, String>>, ErrorResponder> {
        Ok(Json(patterns))
    }

    #[post("/insert", data = "<req>")]
    pub async fn insert_manga(
        context: &State<Arc<Context>>,
        req: Json<MangaRequest>,
        pool: &Db,
    ) -> Result<(), ErrorResponder> {
        update_request(context, &req, pool).await;
        Ok(())
    }
}
