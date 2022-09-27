use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::genre::Genre;
use rocket_db_pools::Connection;

use super::Assemble;

#[async_trait]
impl Assemble for Genre {
    async fn assemble_many(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<Genre>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                Genre,
                "SELECT genre.genre_id as id, genre.name from manga, manga_genre, genre where manga.manga_id = ? AND manga.manga_id = manga_genre.manga_id AND manga_genre.genre_id = genre.genre_id",
                id
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?
        )
    }

    async fn all(conn: &Db) -> Result<Vec<Genre>, ErrorResponder> {
        Ok(sqlx::query_as!(
            Genre,
            "SELECT genre.genre_id as id, genre.name from genre order by genre.name ASC",
        )
        .fetch_all(&**conn)
        .await
        .map_err(Into::into)?)
    }
}
