use crate::{routes::ErrorResponder, Db};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use sqlx::types::chrono::DateTime;

use super::{author::MangaAuthor, genre::MangaGenre, source::MangaSource};
use crate::models::chapter::MangaChapter;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CompleteManga {
    pub main: MainManga,
    pub related: Vec<LinkedManga>,
}

impl CompleteManga {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<CompleteManga, ErrorResponder> {
        let mut mng = CompleteManga::default();

        let ret = MainManga::assemble(id, conn).await?;

        mng.related =
            LinkedManga::assemble_all(&ret.manga_view.linked_id, &ret.manga_view.id, conn).await?;
        mng.main = ret;

        Ok(mng)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MainManga {
    #[serde(flatten)]
    pub manga_view: MangaView,
    pub chapters: Vec<MangaChapter>,
    pub authors: Vec<MangaAuthor>,
    pub artists: Vec<MangaAuthor>,
    pub genres: Vec<MangaGenre>,
}

impl MainManga {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<MainManga, ErrorResponder> {
        let mut ret = MainManga::default();

        ret.manga_view = MangaView::assemble(id, conn).await?;
        ret.genres = MangaGenre::assemble(id, conn).await?;
        ret.authors = MangaAuthor::assemble_author(&id, conn).await?;
        ret.artists = MangaAuthor::assemble_artist(&id, conn).await?;

        ret.chapters = MangaChapter::assemble(&id, conn).await?;

        Ok(ret)
    }
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct LinkedManga {
    #[serde(flatten)]
    manga_view: MangaView,
    #[sqlx(default)]
    chapters: Vec<MangaChapter>,
}

impl LinkedManga {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<LinkedManga, ErrorResponder> {
        let mut ret = LinkedManga::default();

        ret.manga_view = MangaView::assemble(id, conn).await?;
        ret.chapters = MangaChapter::assemble(id, conn).await?;

        Ok(ret)
    }

    pub async fn assemble_all(
        linked_id: &str,
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<LinkedManga>, ErrorResponder> {
        let all = MangaView::assemble_linked(&linked_id, &id, conn).await?;

        let mut ret = Vec::new();

        for i in all {
            let y = MangaChapter::assemble(&i.id, conn).await?;
            ret.push(LinkedManga {
                manga_view: i,
                chapters: y,
            });
        }

        Ok(ret)
    }
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaView {
    #[sqlx(rename = "manga_id")]
    pub id: String,
    pub linked_id: String,
    name: String,
    cover_url: String,
    last_updated: Option<DateTime<chrono::Utc>>,
    description: String,
    status: String,
    source: MangaSource,
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow, Debug)]
pub struct MangaJoinedView {
    #[sqlx(rename = "manga_id")]
    id: String,
    linked_id: String,
    name: String,
    cover_url: String,
    last_updated: Option<DateTime<chrono::Utc>>,
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

impl MangaView {
    pub async fn assemble(
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<MangaView, ErrorResponder> {
        Ok(
            sqlx::query_as("SELECT manga_id, linked_id, manga.name, cover_url, last_updated, description, status, source.source_id, source.name as source_name from manga, source where manga.manga_id = ? AND manga.source_id = source.source_id")
                .bind(id)
                .fetch_one(&mut **conn)
                .await
                .map(|f: MangaJoinedView| f.into())
                .map_err(Into::into)?
        )
    }

    pub async fn assemble_linked(
        linked_id: &str,
        id: &str,
        conn: &mut Connection<Db>,
    ) -> Result<Vec<MangaView>, ErrorResponder> {
        Ok(
            sqlx::query_as("SELECT manga_id, linked_id, manga.name, cover_url, last_updated, description, status, source.source_id, source.name as source_name from manga, source where manga.linked_id = ? AND manga.manga_id != ? AND manga.source_id = source.source_id")
                .bind(linked_id)
                .bind(id)
                .fetch_all(&mut **conn)
                .await
                .map(|f: Vec<MangaJoinedView>| f.into_iter().map(|t| t.into()).collect())
                .map_err(Into::into)?
        )
    }
}
