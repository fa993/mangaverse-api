use std::collections::HashSet;

use crate::{routes::ErrorResponder, Db};
use chrono::NaiveDateTime;
use rocket_db_pools::Connection;

use mangaverse_entity::models::{
    author::Author,
    chapter::MangaChapter,
    genre::Genre,
    manga::{CompleteManga, LinkedManga, MainManga, MangaView},
    source::MangaSource,
};

use super::{
    author::{ArtistOption, AuthorOption},
    Assemble, AssembleWithArgs,
};

#[async_trait]
impl Assemble for CompleteManga {
    async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<CompleteManga, ErrorResponder> {
        let mut mng = CompleteManga::default();

        let ret = MainManga::assemble(id, conn).await?;

        mng.related =
            LinkedManga::all_with_args((&ret.manga_view.id, &ret.manga_view.linked_id), conn)
                .await?;
        mng.main = ret;

        Ok(mng)
    }
}

#[async_trait]
impl Assemble for MainManga {
    async fn assemble(id: &str, conn: &mut Connection<Db>) -> Result<MainManga, ErrorResponder> {
        Ok(MainManga {
            manga_view: MangaView::assemble(id, conn).await?,
            genres: Genre::assemble_many(id, conn).await?,
            authors: Author::assemble_many_with_args(id, AuthorOption, conn).await?,
            artists: Author::assemble_many_with_args(id, ArtistOption, conn).await?,
            chapters: MangaChapter::assemble_many(id, conn).await?,
        })
    }
}

#[async_trait]
impl Assemble for LinkedManga {
    async fn assemble(id: &str, conn: &mut Connection<Db>) -> Result<LinkedManga, ErrorResponder> {
        Ok(LinkedManga {
            manga_view: MangaView::assemble(id, conn).await?,
            chapters: MangaChapter::assemble_many(id, conn).await?,
        })
    }
}

#[async_trait]
impl AssembleWithArgs<(&'_ str, &'_ str)> for LinkedManga {
    async fn all_with_args<'a>(
        ids: (&'a str, &'a str),
        conn: &mut Connection<Db>,
    ) -> Result<Vec<LinkedManga>, ErrorResponder> {
        let id = ids.0;
        let linked_id = ids.1;
        let all = MangaView::assemble_many_with_args(id, linked_id, conn).await?;

        let mut ret = Vec::new();

        for i in all {
            let y = MangaChapter::assemble_many(&i.id, conn).await?;
            ret.push(LinkedManga {
                manga_view: i,
                chapters: y,
            });
        }

        Ok(ret)
    }
}

struct MangaJoinedView {
    id: String,
    linked_id: String,
    name: String,
    cover_url: String,
    last_updated: Option<NaiveDateTime>,
    description: String,
    status: String,
    source_id: String,
    source_name: String,
}

impl From<MangaJoinedView> for MangaView {
    fn from(t: MangaJoinedView) -> Self {
        MangaView {
            id: t.id,
            linked_id: t.linked_id,
            name: t.name,
            cover_url: t.cover_url,
            last_updated: t.last_updated,
            description: t.description,
            status: t.status,
            source: MangaSource {
                id: t.source_id,
                name: t.source_name,
            },
        }
    }
}

#[async_trait]
impl Assemble for MangaView {
    async fn assemble(id: &str, conn: &mut Connection<Db>) -> Result<MangaView, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaJoinedView,
                "SELECT manga_id as id, linked_id, manga.name, cover_url, last_updated, description, status, source.source_id, source.name as source_name from manga, source where manga.manga_id = ? AND manga.source_id = source.source_id",
                id
            )
            .fetch_one(&mut **conn)
            .await
            .map(|f| f.into())?
        )
    }
}

#[async_trait]
impl AssembleWithArgs<&'_ str> for MangaView {
    async fn assemble_many_with_args<'a>(
        id: &'_ str,
        linked_id: &'a str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaView>, ErrorResponder> {
        Ok(
            sqlx::query_as!(
                MangaJoinedView,
                "SELECT manga_id as id, linked_id, manga.name, cover_url, last_updated, description, status, source.source_id, source.name as source_name from manga, source where manga.linked_id = ? AND manga.manga_id != ? AND manga.source_id = source.source_id",
                linked_id, id
            )
            .fetch_all(&mut **conn)
            .await?
            .into_iter().map(Into::into)
            .collect()
        )
    }
}

pub async fn get_urls_from_linked_ids(
    ids: &[String],
    conn: &mut Connection<Db>,
) -> Result<HashSet<String>, ErrorResponder> {
    //this function is slow... optimize target

    let mut u = HashSet::new();

    for t in ids {
        let lt = sqlx::query!("SELECT linked_id from manga where manga_id = ?", t)
            .fetch_one(&mut **conn)
            .await?
            .linked_id;

        u.extend(
            sqlx::query!("SELECT url from manga where linked_id = ?", lt)
                .fetch_all(&mut **conn)
                .await?
                .into_iter()
                .map(|f| f.url),
        );
    }

    Ok(u)
}

pub async fn check_if_manga_exists(url: &str, conn: &mut Connection<Db>) -> Result<bool, sqlx::Error> {
    let y = sqlx::query!("SELECT EXISTS(SELECT manga_id from manga where url = ?) as ex", url)
        .fetch_one(&mut **conn)
        .await?
        .ex;
    Ok(y!=0)
}

//done using temporary table and corrected server side

// struct IdAndGenres {
//     manga_id: String,
//     genres: Option<String>
// }

// pub async fn _corrections(conn: &Db) {
//     sqlx::query!("UPDATE manga_listing INNER JOIN manga ON manga.manga_id = manga_listing.manga_id SET manga_listing.cover_url = manga.cover_url, manga_listing.name = manga.name, manga_listing.description_small = SUBSTR(manga.description, 1, 255)").execute(& **conn).await;
//     let mut i: i32 = 0;
//     loop {
//         let v = sqlx::query_as!(IdAndGenres, "SELECT manga.manga_id as manga_id, group_concat(CONCAT(UPPER(SUBSTRING(genre.name,1,1)),LOWER(SUBSTRING(genre.name,2))) SEPARATOR ', ') as genres from manga, manga_genre, genre, manga_listing where manga_listing.manga_id = manga.manga_id and manga_genre.manga_id = manga.manga_id and genre.genre_id = manga_genre.genre_id group by manga.manga_id limit ?, ?", i, 10 as i32).fetch_all(& **conn).await.expect("Couldn't execute query");

//         if v.len() == 0 {
//             println!("Breaking at i {}", i);
//             break;
//         }

//         for t2 in v {
//             sqlx::query!("UPDATE manga_listing SET genres = ? where manga_id = ?", t2.genres.unwrap_or("".to_string()), t2.manga_id).execute(& **conn).await.expect("Couldnt reinsert genre");
//         }
//         i += 10;

//     }

// }