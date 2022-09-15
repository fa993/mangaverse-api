use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::{routes::ErrorResponder, Db};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MangaQuery {
    id: String,
    name: String,
    offset: u32,
    limit: u32,
    preferred_source_id: Option<String>,
    genre_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, sqlx::FromRow)]
pub struct MangaHeading {
    id: String,
    name: String,
    cover_url: String,
    genres: String,
    small_description: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MangaQueryResponse {
    query: MangaQuery,
    headings: Vec<MangaHeading>,
}

impl MangaQueryResponse {
    pub async fn assemble_query(
        query: MangaQuery,
        conn: &mut Connection<Db>,
    ) -> Result<MangaQueryResponse, ErrorResponder> {
        if query.preferred_source_id.is_none() {
            let mut q_str = String::from('%');
            q_str.push_str(&query.name);
            q_str.push('%');
            let ret: Vec<MangaHeading> = sqlx::query_as(
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.is_main = 1 AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id limit ?, ?",
            )
            .bind(q_str)
            .bind(query.offset)
            .bind(query.limit)
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        } else if let Some(t) = &query.preferred_source_id {
            let mut q_str = String::from('%');
            q_str.push_str(&query.name);
            q_str.push('%');
            let ret: Vec<MangaHeading> = sqlx::query_as(
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.source_id = ? AND manga.is_main IS NOT NULL AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id UNION select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where exists (select linked_id from title where manga.linked_id = title.linked_id AND title.title LIKE ? ) AND manga.is_main = 1 AND manga.is_old = false AND manga_listing.manga_id = manga.manga_id limit ?, ?",
            )
            .bind(q_str.as_str())
            .bind(t)
            .bind(q_str.as_str())
            .bind(query.offset)
            .bind(query.limit)
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

    pub async fn assemble_home(
        mut query: MangaQuery,
        conn: &mut Connection<Db>,
    ) -> Result<MangaQueryResponse, ErrorResponder> {
        if query.genre_ids.is_empty() {
            let ret: Vec<MangaHeading> = sqlx::query_as(
                "select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing where manga.is_main = 1 AND manga.is_old = false AND manga.manga_id = manga_listing.manga_id  order by manga.name ASC limit ?, ?",
            )
            .bind(query.offset)
            .bind(query.limit)
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        } else {
            let vec_len = (query.genre_ids.len() as u32).clone();
            let ret: Vec<MangaHeading> = sqlx::query_as(
                &("select manga_listing.manga_id as id, manga_listing.name as name, manga_listing.cover_url as cover_url, manga_listing.description_small as small_description, manga_listing.genres as genres from manga, manga_listing, manga_genre where manga.manga_id = manga_genre.manga_id AND manga.manga_id = manga_listing.manga_id AND manga_genre.genre_id IN ".to_string() + &MangaQueryResponse::generate_str(query.genre_ids.as_mut_slice()) + " AND manga.is_main = 1 AND manga.is_old = false group by manga.manga_id HAVING count(*) = ? order by manga.name ASC limit ?, ?"),
            )
            .bind(vec_len)
            .bind(query.offset)
            .bind(query.limit)
            .fetch_all(&mut **conn)
            .await
            .map_err(Into::into)?;
            Ok(MangaQueryResponse {
                query,
                headings: ret,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MangaRequest {
    url: String,
    id: String,
}
