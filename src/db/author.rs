use mangaverse_entity::models::author::MangaAuthor;
use crate::{routes::ErrorResponder, Db};
use rocket_db_pools::Connection;

use super::AssembleWithArgs;

pub struct AuthorOption;
pub struct ArtistOption;

#[async_trait]
impl AssembleWithArgs<AuthorOption> for MangaAuthor {
    async fn assemble_many_with_args<'a>(
        id: &'_ str,
        _: AuthorOption,
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
}

#[async_trait]
impl AssembleWithArgs<ArtistOption> for MangaAuthor {
    async fn assemble_many_with_args<'a>(
        id: &str,
        _: ArtistOption,
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
