use rocket::serde::{Deserialize, Serialize};
use sea_orm::{FromQueryResult, prelude::Uuid};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct MangaGenre {
    id: Uuid,
    name: String,
}
