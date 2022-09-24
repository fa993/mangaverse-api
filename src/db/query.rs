use mangaverse_entity::models::query::{MangaHeading, MangaQuery, MangaQueryResponse};
use crate::{Db, routes::ErrorResponder};
use rocket_db_pools::Connection;
use super::{AssembleWithArgs, AssembleWithArgsAndOutput};

#[derive(sqlx::FromRow)]
struct MangaHeadingFromRow {
    id: String,
    name: String,
    cover_url: String,
    genres: String,
    small_description: String,
}

impl Into<MangaHeading> for MangaHeadingFromRow {
    fn into(self) -> MangaHeading {
        MangaHeading {
            id: self.id,
            name: self.name,
            cover_url: self.cover_url,
            genres: self.genres,
            small_description: self.small_description,
        }
    }
}

fn generate_str(genre: &mut [String]) -> String {
    genre.into_iter().for_each(|f| {
        f.insert(0, '"');
        f.push('"');
    });
    let mut st = genre.join(",");
    st.insert(0, '(');
    st.push(')');
    st
}

#[async_trait]
impl AssembleWithArgs<MangaQuery> for MangaQueryResponse {
    async fn assemble_with_args<'a>(
        _: &'_ str,
        query: MangaQuery,
        conn: &mut Connection<Db>,
    ) -> Result<MangaQueryResponse, ErrorResponder> {
        if query.preferred_source_id.is_none() {
            let mut q_str = String::from('%');
            q_str.push_str(&query.name.as_ref().ok_or(ErrorResponder {
                message: "No Name found".to_string(),
            })?);
            q_str.push('%');
            let ret: Vec<MangaHeading> = sqlx::query_as!(
                MangaHeading,
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.is_main = 1 AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id limit ?, ?",
                q_str, query.offset, query.limit
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        } else if let Some(t) = &query.preferred_source_id {
            let mut q_str = String::from('%');
            q_str.push_str(&query.name.as_ref().ok_or(ErrorResponder {
                message: "No Name found".to_string(),
            })?);
            q_str.push('%');
            let ret: Vec<MangaHeading> = sqlx::query_as!(
                MangaHeading,
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.source_id = ? AND manga.is_main IS NOT NULL AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id UNION select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.is_main = 1 AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id limit ?, ?",
                q_str, t, q_str, query.offset, query.limit
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        } else {
            Err(ErrorResponder {
                message: "Internal Database error".to_string(),
            })
        }
    }

}

#[async_trait]
impl AssembleWithArgsAndOutput<MangaQuery, MangaQueryResponse> for MangaQueryResponse {

    async fn all_with_args_and_output<'a>(
        mut query: MangaQuery,
        conn: &mut Connection<Db>,
    ) -> Result<MangaQueryResponse, ErrorResponder> {
        if query.genre_ids.is_empty() {
            let ret: Vec<MangaHeading> = sqlx::query_as!(
                MangaHeading,
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where manga.is_main = 1 AND manga.is_old = false AND manga.manga_id = manga_listing.manga_id  order by manga.name ASC limit ?, ?",
                query.offset, query.limit
            )
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        } else {
            let vec_len = (query.genre_ids.len() as u32).clone();
            let ret: Vec<MangaHeadingFromRow> = sqlx::query_as(
                &("select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing, manga_genre where manga.manga_id = manga_genre.manga_id AND manga.manga_id = manga_listing.manga_id AND manga_genre.genre_id IN ".to_string() + &generate_str(query.genre_ids.as_mut_slice()) + " AND manga.is_main = 1 AND manga.is_old = false group by manga.manga_id HAVING count(*) = ? order by manga.name ASC limit ?, ?"),
            )
            .bind(vec_len)
            .bind(query.offset)
            .bind(query.limit)
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret.into_iter().map(Into::into).collect(),
            })
        }
    }

}
