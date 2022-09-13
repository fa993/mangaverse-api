use chrono::DateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct MangaChapter {
    #[sqlx(rename = "chapter_id")]
    id: String,
    chapter_name: String,
    chapter_number: String,
    sequence_number: i64,
    updated_at: DateTime<chrono::Utc>,
}

impl MangaChapter {
    pub async fn assemble(
        id: &String,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaChapter>, ErrorResponder> {
        Ok(sqlx::query_as("SELECT chapter_id, chapter_name, chapter_number, sequence_number, updated_at from chapter where manga_id = ? ")
                            .bind(id)
                            .fetch_all(&mut **conn)
                            .await
                            .map_err(Into::into)?)
    }
}
