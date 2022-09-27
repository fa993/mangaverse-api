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
        let mut ret = MainManga::default();

        ret.manga_view = MangaView::assemble(id, conn).await?;
        ret.genres = Genre::assemble_many(id, conn).await?;
        ret.authors = Author::assemble_many_with_args(&id, AuthorOption, conn).await?;
        ret.artists = Author::assemble_many_with_args(&id, ArtistOption, conn).await?;

        ret.chapters = MangaChapter::assemble_many(&id, conn).await?;

        Ok(ret)
    }
}

#[async_trait]
impl Assemble for LinkedManga {
    async fn assemble(id: &str, conn: &mut Connection<Db>) -> Result<LinkedManga, ErrorResponder> {
        let mut ret = LinkedManga::default();

        ret.manga_view = MangaView::assemble(id, conn).await?;
        ret.chapters = MangaChapter::assemble_many(id, conn).await?;

        Ok(ret)
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
        let all = MangaView::assemble_many_with_args(&id, &linked_id, conn).await?;

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
            .map(|f: MangaJoinedView| f.into())
            .map_err(Into::into)?
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
            .await
            .map(|f: Vec<MangaJoinedView>| f.into_iter().map(|t| t.into()).collect())
            .map_err(Into::into)?
        )
    }
}
