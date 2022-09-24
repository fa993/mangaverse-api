use mangaverse_entity::models::chapter::MangaChapter;
use crate::{routes::ErrorResponder, Db};
use rocket_db_pools::Connection;

use super::Assemble;

#[async_trait]
impl Assemble for MangaChapter {
    async fn assemble_many(
        id: &'_ str,
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
