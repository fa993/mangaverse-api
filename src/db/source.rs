use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::source::MangaSource;
use rocket_db_pools::Connection;

use super::Assemble;

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
            .await
            .map_err(Into::into)?
        )
    }
}
