use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::page::{ChapterPosition, PageURL};
use rocket_db_pools::Connection;

use super::{Assemble, AssembleWithArgs};

#[async_trait]
impl Assemble for PageURL {
    async fn assemble_many(
        id: &'_ str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<PageURL>, ErrorResponder> {
        Ok(sqlx::query_as!(
            PageURL,
            "SELECT url from chapter_page where chapter_id = ? order by page_number ASC",
            id
        )
        .fetch_all(&mut **conn)
        .await?)
    }
}

#[async_trait]
impl AssembleWithArgs<u32> for ChapterPosition {
    async fn assemble_with_args<'a>(
        id: &'_ str,
        seq: u32,
        conn: &mut Connection<Db>,
    ) -> Result<ChapterPosition, ErrorResponder> {
        struct DatabaseNum {
            pub num: i64,
        }

        let idx = sqlx::query_as!(
                DatabaseNum,
                "SELECT COUNT(chapter_page_id) as num FROM chapter_page WHERE exists (SELECT chapter_id FROM chapter WHERE chapter_page.chapter_id = chapter.chapter_id AND manga_id = ? AND sequence_number < ? )",
                id, seq
            )
            .fetch_one(&mut **conn).await?;

        let len = sqlx::query_as!(
                DatabaseNum,
                "SELECT COUNT(chapter_page_id) as num FROM chapter_page WHERE exists (SELECT chapter_id FROM chapter WHERE chapter_page.chapter_id = chapter.chapter_id AND manga_id = ? )",
                id
            )
            .fetch_one(&mut **conn).await?;

        Ok(ChapterPosition {
            index: idx.num,
            length: len.num,
        })
    }
}
