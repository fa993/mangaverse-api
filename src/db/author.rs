use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::author::Author;
use rocket_db_pools::Connection;

use super::AssembleWithArgs;

pub struct AuthorOption;
pub struct ArtistOption;

#[async_trait]
impl AssembleWithArgs<AuthorOption> for Author {
    async fn assemble_many_with_args<'a>(
        id: &'_ str,
        _: AuthorOption,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<Author>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                Author,
                "SELECT author.author_id as id, author.name from manga, manga_author, author where manga.manga_id = ? AND manga.manga_id = manga_author.manga_id AND manga_author.author_id = author.author_id", 
                id
            )
            .fetch_all(&mut **conn)
            .await?
        )
    }
}

#[async_trait]
impl AssembleWithArgs<ArtistOption> for Author {
    async fn assemble_many_with_args<'a>(
        id: &str,
        _: ArtistOption,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<Author>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                Author,
                "SELECT author.author_id as id, author.name from manga, manga_artist, author where manga.manga_id = ? AND manga.manga_id = manga_artist.manga_id AND manga_artist.author_id = author.author_id",
                id
            )
            .fetch_all(&mut **conn)
            .await?
        )
    }
}
