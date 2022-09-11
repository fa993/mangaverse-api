use crate::{routes::ErrorResponder, Db};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::types::chrono::DateTime;

use super::{
    author::MangaAuthor,
    genre::MangaGenre,
    source::{LinkedMangaSource, MainMangaSource},
};
use crate::models::chapter::MangaChapter;

#[derive(Serialize, Deserialize, Default)]
pub struct CompleteManga {
    pub main: MainManga,
    pub related: Vec<LinkedManga>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MainManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub source: MainMangaSource,
    pub chapters: Vec<MangaChapter>,
    pub authors: Vec<MangaAuthor>,
    pub artists: Vec<MangaAuthor>,
    pub genres: Vec<MangaGenre>,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow)]
pub struct LinkedManga {
    #[serde(flatten)]
    manga_view: MangaView,
    source: LinkedMangaSource,
    #[sqlx(default)]
    chapters: Vec<MangaChapter>,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaView {
    #[sqlx(rename = "manga_id")]
    id: String,
    linked_id: String,
    name: String,
    cover_url: String,
    last_updated: Option<DateTime<chrono::Utc>>,
    description: String,
    status: String,
}

impl MangaView {
    pub async fn assemble(
        id: &String,
        conn: &mut Connection<Db>,
    ) -> Result<MangaView, ErrorResponder> {
        Ok(sqlx::query_as("SELECT manga_id, linked_id, name, cover_url, last_updated, description, status from manga where manga.manga_id = ?")
                            .bind(id)
                            .fetch_one(&mut **conn)
                            .await
                            .map_err(Into::into)?)
    }
}
