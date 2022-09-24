use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaSource {
    pub id: String,
    pub name: String,
}

impl MangaSource {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<MangaSource, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaSource,
                "SELECT source.source_id as id, source.name from source, manga where manga.source_id = source.source_id AND manga.manga_id = ?",
                id
            )
            .fetch_one(&mut **conn)
            .await
            .map_err(Into::into)?
        )
    }
}
