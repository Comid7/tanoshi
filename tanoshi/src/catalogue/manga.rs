use crate::context::GlobalContext;
use async_graphql::{Context, Object, Enum};
use url::Url;

/// A type represent manga details, normalized across source
pub struct Manga {
    pub id: i32,
    pub hash: String,
    pub title: String,
    pub author: Vec<String>,
    pub genre: Vec<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub url: Url,
    pub thumbnail_url: Url,
    pub last_read: Option<i32>,
    pub last_page: Option<i32>,
    pub is_favorite: bool,
}

#[Object]
impl Manga {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn hash(&self) -> String {
        self.hash.clone()
    }

    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn author(&self) -> Vec<String> {
        self.author.clone()
    }

    async fn genre(&self) -> Vec<String> {
        self.genre.clone()
    }

    async fn status(&self) -> Option<String> {
        self.status.clone()
    }

    async fn description(&self) -> Option<String> {
        self.description.clone()
    }

    async fn url(&self) -> String {
        self.url.as_str().to_string()
    }

    async fn thumbnail_url(&self) -> String {
        self.thumbnail_url.as_str().to_string()
    }

    async fn last_read(&self) -> Option<i32> {
        self.last_read
    }

    async fn last_page(&self) -> Option<i32> {
        self.last_page
    }

    async fn is_favorite(&self) -> bool {
        self.is_favorite
    }
}

impl From<&tanoshi_lib::model::Manga> for Manga {
    fn from(m: &tanoshi_lib::model::Manga) -> Self {
        Self {
            id: m.id,
            hash: m.hash.clone(),
            title: m.title.clone(),
            author: m.author.clone(),
            genre: m.genre.clone(),
            status: m.status.clone(),
            description: m.description.clone(),
            url: m.url.clone(),
            thumbnail_url: m.thumbnail_url.clone(),
            last_page: None,
            last_read: None,
            is_favorite: false,
        }
    }
}

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
pub struct MangaRoot;

#[Object]
impl MangaRoot {
    async fn mangas(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "source of the manga")] source: String,
        #[graphql(desc = "keyword of the manga")] keyword: Option<String>,
        #[graphql(desc = "genres of the manga")] genres: Option<Vec<String>>,
        #[graphql(desc = "page")] page: Option<i32>,
        #[graphql(desc = "sort by")] sort_by: Option<SortByParam>,
        #[graphql(desc = "sort order")] sort_order: Option<SortOrderParam>,
    ) -> Vec<Manga> {
        let sort_by = sort_by.map(|s| s.into());
        let sort_order = sort_order.map(|s| s.into());
        ctx.data_unchecked::<GlobalContext>()
            .extensions
            .get(&source)
            .unwrap()
            .get_mangas(keyword, genres, page, sort_by, sort_order, None)
            .await
            .unwrap()
            .iter()
            .map(|m| m.into())
            .collect()
    }
}
