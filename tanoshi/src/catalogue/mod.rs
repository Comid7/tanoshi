mod source;
pub use source::{Source, SourceRoot};

mod manga;
pub use manga::Manga;

mod chapter;
pub use chapter::Chapter;

mod page;
pub use page::Page;

use crate::context::GlobalContext;

use async_graphql::futures::{stream, StreamExt};
use async_graphql::{Context, Enum, Object, Result};

/// A type represent sort parameter for query manga from source, normalized across sources
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "tanoshi_lib::model::SortByParam")]
pub enum SortByParam {
    LastUpdated,
    Title,
    Comment,
    Views,
}

/// A type represent order parameter for query manga from source, normalized across sources
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "tanoshi_lib::model::SortOrderParam")]
pub enum SortOrderParam {
    Asc,
    Desc,
}

#[derive(Default)]
pub struct CatalogueRoot;

#[Object]
impl CatalogueRoot {
    async fn browse_source(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source id")] source_id: i64,
        #[graphql(desc = "keyword of the manga")] keyword: Option<String>,
        #[graphql(desc = "genres of the manga")] genres: Option<Vec<String>>,
        #[graphql(desc = "page")] page: Option<i32>,
        #[graphql(desc = "sort by")] sort_by: Option<SortByParam>,
        #[graphql(desc = "sort order")] sort_order: Option<SortOrderParam>,
    ) -> Vec<Manga> {
        let sort_by = sort_by.map(|s| s.into());
        let sort_order = sort_order.map(|s| s.into());
        let db = ctx.data_unchecked::<GlobalContext>().db.clone();
        let mangas = ctx
            .data_unchecked::<GlobalContext>()
            .extensions
            .get(source_id)
            .unwrap()
            .get_mangas(keyword, genres, page, sort_by, sort_order, None)
            .await
            .unwrap();
        let mangas_stream = stream::iter(mangas);
        let mangas_stream = mangas_stream.then(|m| async {
            match db.get_manga_by_source_path(source_id, &m.path).await {
                Some(manga) => manga,
                None => {
                    let mut manga: Manga = m.into();
                    let manga_id = db.insert_manga(&manga).await.unwrap();
                    manga.id = manga_id;
                    manga
                }
            }
        });
        mangas_stream.collect().await
    }

    async fn manga(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "manga id")] id: i64,
    ) -> Option<Manga> {
        let db = ctx.data_unchecked::<GlobalContext>().db.clone();
        if let Some(manga) = db.get_manga_by_id(id).await {
            if manga.incomplete() {
                let mut m: Manga = ctx
                    .data_unchecked::<GlobalContext>()
                    .extensions
                    .get(manga.source_id)
                    .unwrap()
                    .get_manga_info(manga.path.clone())
                    .await
                    .ok()
                    .map(|m| m.into())
                    .unwrap();

                m.id = id;
                m.date_added = manga.date_added;
                db.update_manga_info(&m).await.unwrap();
                Some(m)
            } else {
                Some(manga)
            }
        } else {
            None
        }
    }
}