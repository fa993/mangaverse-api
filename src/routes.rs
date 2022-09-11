use std::convert::Infallible;

#[derive(Responder)]
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

    use crate::models::genre::MangaGenre;
    use crate::models::manga::{CompleteManga, MainManga, MangaView};
    use crate::models::source::MangaSource;
    use crate::routes::ErrorResponder;
    use crate::Db;
    use rocket::serde::json::Json;
    use rocket::serde::uuid::Uuid;
    use rocket_db_pools::Connection;

    #[get("/v1/<id>")]
    pub async fn get_manga(
        mut conn: Connection<Db>,
        id: Uuid,
    ) -> Result<Json<CompleteManga>, ErrorResponder> {
        let act_id = id.to_string();

        let mut ret = MainManga::default();

        ret.manga_view = MangaView::assemble(&act_id, &mut conn).await?;
        ret.source = MangaSource::assemble(&act_id, &mut conn).await?;
        ret.genres = MangaGenre::assemble(&act_id, &mut conn).await?;
        // ret.artists
        // ret.authors
        // ret.chapters

        println!("{:#?}", ret);

        // println!("{:#?}", genres);
        let mut mng = CompleteManga::default();

        mng.main = ret;

        Ok(Json(mng))
    }
}
