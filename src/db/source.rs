use crate::{models::source::MangaSource, routes::ErrorResponder, Db};
use rocket_db_pools::Connection;

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
