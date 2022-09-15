use std::collections::HashMap;

use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct SourcePattern {
    url: String,
    source_id: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AllPatterns {
    pub patterns: HashMap<String, String>,
}

impl SourcePattern {
    pub async fn all(conn: &crate::Db) -> Result<AllPatterns, crate::routes::ErrorResponder> {
        Ok(AllPatterns {
            patterns: sqlx::query_as("SELECT source_id, url from source_pattern")
                .fetch_all(&**conn)
                .await
                .map_err(Into::into)?
                .into_iter()
                .map(|t: SourcePattern| (t.url, t.source_id))
                .collect(),
        })
    }
}
