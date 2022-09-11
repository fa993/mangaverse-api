use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

pub type MainMangaSource = MangaSource;
pub type LinkedMangaSource = MangaSource;

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaSource {
    #[sqlx(rename = "source_id")]
    id: String,
    name: String,
}

impl MangaSource {
    pub async fn assemble(
        id: &String,
        conn: &mut Connection<Db>,
    ) -> Result<MangaSource, ErrorResponder> {
        Ok(sqlx::query_as("SELECT source.source_id, source.name from source, manga where manga.source_id = source.source_id AND manga.manga_id = ?")
        .bind(id)
        .fetch_one(&mut **conn)
        .await
        .map_err(Into::into)?)
    }
}
