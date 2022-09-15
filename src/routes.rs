use std::convert::Infallible;

#[derive(Responder, Debug)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    pub message: String,
}

#[allow(clippy::from_over_into)]
impl Into<ErrorResponder> for sqlx::Error {
    fn into(self) -> ErrorResponder {
        ErrorResponder {
            message: self.to_string(),
        }
    }
}

impl Into<ErrorResponder> for Infallible {
    fn into(self) -> ErrorResponder {
        ErrorResponder {
            message: self.to_string(),
        }
    }
}

pub mod v1 {

    use std::collections::HashMap;
    use std::sync::Arc;

    use crate::models::genre::MangaGenre;
    use crate::models::manga::{CompleteManga, LinkedManga};
    use crate::models::page::{ChapterPosition, PageURL};
    use crate::models::pattern::AllPatterns;
    use crate::models::query::{MangaQuery, MangaQueryResponse, MangaRequest};
    use crate::routes::ErrorResponder;
    use crate::Db;
    use rocket::serde::json::Json;
    use rocket::serde::uuid::Uuid;
    use rocket::State;
    use rocket_db_pools::Connection;

    #[get("/v1/<id>")]
    pub async fn get_manga(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<CompleteManga>, ErrorResponder> {
        Ok(Json(CompleteManga::assemble(id.to_string().as_str(), &mut conn).await?))
    }

    #[post("/v1/refresh", data = "<_ids>")]
    pub fn refresh_all(_ids: Json<Vec<Uuid>>) -> Result<(), ErrorResponder> {
        Ok(())
    }

    #[get("/v1/part/<id>")]
    pub async fn get_linked_manga(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<LinkedManga>, ErrorResponder> {
        Ok(Json(
            LinkedManga::assemble(&id.to_string(), &mut conn).await?,
        ))
    }

    #[get("/v1/genres")]
    pub async fn get_all_genres(
        genres: &State<Arc<Vec<MangaGenre>>>,
    ) -> Result<Json<&[MangaGenre]>, ErrorResponder> {
        Ok(Json(genres.inner().as_slice()))
    }

    #[get("/v1/chapter/<id>")]
    pub async fn get_chapter(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<Vec<PageURL>>, ErrorResponder> {
        Ok(Json(PageURL::assemble(&id.to_string(), &mut conn).await?))
    }

    #[get("/v1/chapter/position/<manga_id>/<sequence_number>")]
    pub async fn get_chapter_position(
        mut conn: Connection<Db>,
        manga_id: Uuid,
        sequence_number: u32,
    ) -> Result<Json<ChapterPosition>, ErrorResponder> {
        Ok(Json(
            ChapterPosition::assemble(&manga_id.to_string(), sequence_number, &mut conn).await?,
        ))
    }

    #[post("/v1/search", data = "<query>")]
    pub async fn get_manga_from_query(
        query: Json<MangaQuery>,
        mut conn: Connection<Db>,
    ) -> Result<Json<MangaQueryResponse>, ErrorResponder> {
        Ok(Json(
            MangaQueryResponse::assemble_query(query.0, &mut conn).await?,
        ))
    }

    #[post("/v1/home", data = "<query>")]
    pub async fn get_manga_for_home(
        query: Json<MangaQuery>,
        mut conn: Connection<Db>,
    ) -> Result<Json<MangaQueryResponse>, ErrorResponder> {
        Ok(Json(
            MangaQueryResponse::assemble_home(query.0, &mut conn).await?,
        ))
    }

    #[get("/v1/currentSources")]
    pub async fn get_source_patterns(
        patterns: &State<Arc<AllPatterns>>,
    ) -> Result<Json<HashMap<String, String>>, ErrorResponder> {
        let pts: HashMap<String, String> = patterns.inner().patterns.clone();
        Ok(Json(pts))
    }

    #[post("/v1/insert", data = "<_req>")]
    pub fn insert_manga(_req: Json<MangaRequest>) -> Result<(), ErrorResponder> {
        Ok(())
    }
}
