use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaQuery {
    pub id: String,
    pub name: Option<String>,
    pub offset: u32,
    pub limit: u32,
    pub preferred_source_id: Option<String>,
    #[serde(default)]
    pub genre_ids: Vec<String>,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MangaHeading {
    pub id: String,
    pub name: String,
    #[serde(rename = "coverURL")]
    pub cover_url: String,
    pub genres: String,
    pub small_description: String,
}

#[derive(Serialize, Default, Debug)]
pub struct MangaQueryResponse {
    pub query: MangaQuery,
    pub headings: Vec<MangaHeading>,
}

#[derive(Deserialize, Default)]
#[allow(dead_code)]
pub struct MangaRequest {
    pub url: String,
    pub id: String,
}
