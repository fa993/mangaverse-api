use std::collections::HashMap;

use crate::{routes::ErrorResponder, Db};
use mangaverse_entity::models::pattern::SourcePattern;

use super::AssembleWithOutput;

#[async_trait]
impl AssembleWithOutput<HashMap<String, String>> for SourcePattern {
    async fn all_with_output(conn: &Db) -> Result<HashMap<String, String>, ErrorResponder> {
        Ok(
            sqlx::query_as!(SourcePattern, "SELECT source_id, url from source_pattern")
                .fetch_all(&**conn)
                .await?
                .into_iter()
                .map(|t: SourcePattern| (t.url, t.source_id))
                .collect(),
        )
    }
}
