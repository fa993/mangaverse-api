use std::collections::HashMap;

use rocket::serde::Serialize;

pub struct AllPatterns {
    pub patterns: HashMap<String, String>,
}

#[derive(Serialize, Default, Debug)]
pub struct SourcePattern {
    pub url: String,
    pub source_id: String,
}
