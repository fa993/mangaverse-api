use crate::{models::chapter::MangaChapter, routes::ErrorResponder, Db};
use rocket_db_pools::Connection;

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
