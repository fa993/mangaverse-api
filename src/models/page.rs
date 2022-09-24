use rocket::serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct PageURL {
    pub url: String,
}

#[derive(Serialize, Default, Debug)]
pub struct ChapterPosition {
    pub index: i64,
    pub length: i64,
}
