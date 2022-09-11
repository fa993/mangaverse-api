use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaChapter {
    id: String,
    name: String,
}
