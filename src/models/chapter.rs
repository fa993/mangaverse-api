use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaChapter {
    pub id: String,
    pub chapter_name: String,
    pub chapter_number: String,
    pub sequence_number: i32,
    pub updated_at: Option<NaiveDateTime>,
}
