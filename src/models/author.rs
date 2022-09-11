use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaAuthor {
    id: String,
    name: String,
}
