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

    use crate::models::manga::CompleteManga;
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

        let mng = CompleteManga::assemble(act_id, &mut conn).await?;

        // let ret = MainManga::assemble(act_id, &mut conn).await?;

        // // println!("{:#?}", genres);
        // let mut mng = CompleteManga::default();

        // mng.related = LinkedManga::assemble(&ret.manga_view.linked_id, &ret.manga_view.id, &mut conn).await?;
        // mng.main = ret;

        println!("{:#?}", mng);

        Ok(Json(mng))
    }
}
