use crate::{routes::ErrorResponder, Db};
use rocket::serde::Serialize;
use rocket_db_pools::Connection;

#[derive(Serialize, Default, Debug)]
pub struct PageURL {
    pub url: String,
}

impl PageURL {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<PageURL>, ErrorResponder> {
        Ok(sqlx::query_as!(
            PageURL,
            "SELECT url from chapter_page where chapter_id = ? order by page_number ASC",
            id
        )
        .fetch_all(&mut **conn)
        .await
        .map_err(Into::into)?)
    }
}

#[derive(Serialize, Default, Debug)]
pub struct ChapterPosition {
    pub index: i64,
    pub length: i64,
}

struct DatabaseNum {
    pub num: i64,
}

impl ChapterPosition {
    pub async fn assemble(
        id: &str,
        seq: u32,
        conn: &mut Connection<Db>,
    ) -> Result<ChapterPosition, ErrorResponder> {
        let idx : DatabaseNum = sqlx::query_as!(
                DatabaseNum,
                "SELECT COUNT(chapter_page_id) as num FROM chapter_page WHERE exists (SELECT chapter_id FROM chapter WHERE chapter_page.chapter_id = chapter.chapter_id AND manga_id = ? AND sequence_number < ? )",
                id, seq
            )
            .fetch_one(&mut **conn)
            .await
            .map_err(Into::into)?;

        let len : DatabaseNum = sqlx::query_as!(
                DatabaseNum,
                "SELECT COUNT(chapter_page_id) as num FROM chapter_page WHERE exists (SELECT chapter_id FROM chapter WHERE chapter_page.chapter_id = chapter.chapter_id AND manga_id = ? )",
                id
            )
            .fetch_one(&mut **conn)
            .await
            .map_err(Into::into)?;

        Ok(ChapterPosition {
            index: idx.num,
            length: len.num,
        })
    }
}
