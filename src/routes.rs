use std::convert::Infallible;

#[derive(Responder, Debug)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
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

    use std::sync::Arc;

    use crate::models::genre::MangaGenre;
    use crate::models::manga::{CompleteManga, LinkedManga};
    use crate::models::page::PageURL;
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
        let act_id = id.to_string();

        let mng = CompleteManga::assemble(act_id, &mut conn).await?;

        println!("{:#?}", mng);

        Ok(Json(mng))
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

    /*

    @GetMapping("/chapter/position/{manga_id}/{sequence_number}")
    public ChapterPosition getChapterIndex(@PathVariable(name = "manga_id") String mangaId, @PathVariable(name = "sequence_number") Integer sequenceNumber) {
        return this.pageManager.getPosition(mangaId, sequenceNumber);
    }

    @PostMapping("/search")
    public MangaQueryResponse getManga(@RequestBody MangaQuery query) {
        return new MangaQueryResponse(query, listingManager.findAllByQuery(query));
    }

    @PostMapping("/home")
    public MangaQueryResponse getHomePage(@RequestBody MangaQuery query) {
        return new MangaQueryResponse(query, listingManager.getHome(query));
    }

    @GetMapping("/currentSources")
    public Map<String, String> getSourcePatterns() {
        return sourceManager.getPatterns();
    }

    @PostMapping("/insert")
    public void insertURL(@RequestBody MangaRequest req) {
        sct.watchSingle(req.url, req.id);
    }

    */
}
