use sea_orm::{FromQueryResult, prelude::Uuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct MangaAuthor {
    id: Uuid,
    name: String,
}
