use chrono::DateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct MangaChapter {
    #[sqlx(rename = "chapter_id")]
    id: String,
    #[serde(rename = "chapterName")]
    chapter_name: String,
    #[serde(rename = "chapterNumber")]
    chapter_number: String,
    #[serde(rename = "sequenceNumber")]
    sequence_number: i64,
    #[serde(rename = "updatedAt")]
    updated_at: DateTime<chrono::Utc>,
}

impl MangaChapter {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaChapter>, ErrorResponder> {
        Ok(
            sqlx::query_as("SELECT chapter_id, chapter_name, chapter_number, sequence_number, updated_at from chapter where manga_id = ? ")
                .bind(id)
                .fetch_all(&mut **conn)
                .await
                .map_err(Into::into)?
        )
    }
}
