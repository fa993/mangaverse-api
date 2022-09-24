use crate::{models::author::MangaAuthor, routes::ErrorResponder, Db};
use rocket_db_pools::Connection;

impl MangaAuthor {
    pub async fn assemble_author(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaAuthor>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaAuthor,
                "SELECT author.author_id as id, author.name from manga, manga_author, author where manga.manga_id = ? AND manga.manga_id = manga_author.manga_id AND manga_author.author_id = author.author_id", 
                id
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?
        )
    }

    pub async fn assemble_artist(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaAuthor>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaAuthor,
                "SELECT author.author_id as id, author.name from manga, manga_artist, author where manga.manga_id = ? AND manga.manga_id = manga_artist.manga_id AND manga_artist.author_id = author.author_id",
                id
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?
        )
    }
}