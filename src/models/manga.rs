use rocket::serde::{Deserialize, Serialize};
use sea_orm::{FromQueryResult, TryGetableFromJson, TryGetable, prelude::Uuid};

use super::{author::MangaAuthor, genre::MangaGenre, source::{LinkedMangaSource, MainMangaSource}};
use crate::models::chapter::MangaChapter;

#[derive(Serialize, Deserialize)]
pub struct CompleteManga {
    main: MainManga,
    related: Vec<LinkedManga>,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct MainManga {
    id: Uuid,
    linked_id: Uuid,
    name: String,
    cover_url: String,
    source: MainMangaSource,
    chapters: Vec<MangaChapter>,
    authors: Vec<MangaAuthor>,
    artists: Vec<MangaAuthor>,
    last_updated: chrono::DateTime<chrono::Local>,
    description: String,
    genres: Vec<MangaGenre>,
    status: String,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct LinkedManga {
    id: Uuid,
    linked_id: Uuid,
    name: String,
    cover_url: String,
    source: LinkedMangaSource,
    chapters: Vec<MangaChapter>,
    last_updated: chrono::DateTime<chrono::Local>,
    status: String,
}


