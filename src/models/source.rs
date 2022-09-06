use rocket::serde::{Deserialize, Serialize};
use sea_orm::{FromQueryResult, prelude::Uuid};

pub type MainMangaSource = MangaSource;
pub type LinkedMangaSource = MangaSource;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct MangaSource {
    id: Uuid,
    name: String,
}
