use std::convert::Infallible;

#[derive(Responder)]
#[response(status = 500, content_type = "json")]
pub struct ErrorResponder {
    message: String,
}

#[allow(clippy::from_over_into)]
impl Into<ErrorResponder> for sea_orm::DbErr {
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

    use crate::entities::*;
    use crate::{
        db::pool::Db,
        models::manga::{CompleteManga},
        routes::ErrorResponder,
    };
    use rocket::serde::json::Json;
    use sea_orm::EntityTrait;
    use sea_orm::prelude::Uuid;
    use sea_orm_rocket::Connection;

    #[get("/v1/<id>")]
    pub async fn get_manga(
        conn: Connection<'_, Db>,
        id: Uuid,
    ) -> Result<Json<CompleteManga>, ErrorResponder> {
        let db = conn.into_inner();
        let res = manga::Entity::find_by_id(id.to_string())
            .one(db)
            .await
            .map_err(Into::into)?;

        todo!()
    }
}
