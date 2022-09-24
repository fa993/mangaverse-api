use rocket::serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct MangaSource {
    pub id: String,
    pub name: String,
}
