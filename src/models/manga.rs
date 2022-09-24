use chrono::NaiveDateTime;
use rocket::serde::Serialize;

use super::{author::MangaAuthor, chapter::MangaChapter, genre::MangaGenre, source::MangaSource};

#[derive(Serialize, Default, Debug)]
pub struct CompleteManga {
    pub main: MainManga,
    pub related: Vec<LinkedManga>,
}

#[derive(Serialize, Default, Debug)]
pub struct MainManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub chapters: Vec<MangaChapter>,
    pub authors: Vec<MangaAuthor>,
    pub artists: Vec<MangaAuthor>,
    pub genres: Vec<MangaGenre>,
}

#[derive(Serialize, Default, Debug)]
pub struct LinkedManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub chapters: Vec<MangaChapter>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaView {
    pub id: String,
    pub linked_id: String,
    pub name: String,
    #[serde(rename = "coverURL")]
    pub cover_url: String,
    pub last_updated: Option<NaiveDateTime>,
    pub description: String,
    pub status: String,
    pub source: MangaSource,
}
