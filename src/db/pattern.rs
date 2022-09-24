use mangaverse_entity::models::pattern::{AllPatterns, SourcePattern};
use crate::{Db, routes::ErrorResponder};

use super::AssembleWithOutput;

#[async_trait]
impl AssembleWithOutput<AllPatterns> for SourcePattern {
    async fn all_with_output(conn: &Db) -> Result<AllPatterns, ErrorResponder> {
        Ok(AllPatterns {
            patterns: sqlx::query_as!(SourcePattern, "SELECT source_id, url from source_pattern")
                .fetch_all(&**conn)
                .await
                .map_err(Into::into)?
                .into_iter()
                .map(|t: SourcePattern| (t.url, t.source_id))
                .collect(),
        })
    }
}
