use chrono::NaiveDateTime;
use rocket::serde::Serialize;
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapter {
    id: String,
    chapter_name: String,
    chapter_number: String,
    sequence_number: i32,
    updated_at: Option<NaiveDateTime>,
}

impl MangaChapter {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaChapter>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaChapter,
                "SELECT chapter_id as id, chapter_name, chapter_number, sequence_number, updated_at from chapter where manga_id = ? ",
                id
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?
        )
    }
}
