use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct MangaAuthor {
    pub id: String,
    pub name: String,
}
