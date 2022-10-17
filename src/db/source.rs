use std::collections::HashMap;

use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::source::{MangaSource, SourceTable};
use rocket_db_pools::Connection;

use super::{Assemble, AssembleWithOutput};

#[async_trait]
impl Assemble for MangaSource {
    async fn assemble(
        id: &'_ str,
        conn: &mut Connection<Db>,
    ) -> Result<MangaSource, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaSource,
                "SELECT source.source_id as id, source.name from source, manga where manga.source_id = source.source_id AND manga.manga_id = ?",
                id
            )
            .fetch_one(&mut **conn)
            .await?
        )
    }
}

#[async_trait]
impl AssembleWithOutput<HashMap<String, SourceTable>> for SourceTable {
    async fn all_with_output(conn: &Db) -> Result<HashMap<String, SourceTable>, ErrorResponder> {
        Ok(sqlx::query_as!(
            SourceTable,
            "SELECT source_id as id, name, priority from source"
        )
        .fetch_all(&**conn)
        .await?
        .into_iter()
        .map(|t| (t.id.clone(), t))
        .collect())
    }
}
