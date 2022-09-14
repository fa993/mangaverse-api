use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct PageURL {
    pub url: String,
}

impl PageURL {
    pub async fn assemble(
        id: &String,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<PageURL>, ErrorResponder> {
        Ok(sqlx::query_as(
            "SELECT url from chapter_page where chapter_id = ? order by page_number ASC",
        )
        .bind(id)
        .fetch_all(&mut **conn)
        .await
        .map_err(Into::into)?)
    }
}

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct ChapterPage {
    pub index: u128,
    pub length: u128,
}
