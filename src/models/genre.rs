use rocket::serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct MangaGenre {
    pub id: String,
    pub name: String,
}
