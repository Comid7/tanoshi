use async_graphql::{Context, Object, Result, Schema, Subscription, ID};

/// A type represent manga details, normalized across source
pub struct Manga {
    pub id: i32,
    pub source: String,
    pub title: String,
    pub author: Vec<String>,
    pub genre: Vec<String>,
    pub status: Option<String>,
    pub description: Option<String>,
    pub path: String,
    pub thumbnail_url: String,
    pub last_read: Option<i32>,
    pub last_page: Option<i32>,
    pub is_favorite: bool,
}

#[Object]
impl Manga {
    async fn id(&self) ->  i32 {
        self.id
	}

    async fn source(&self) ->  String {
        self.source.clone()
	}

    async fn title(&self) ->  String {
        self.title.clone()
	}

    async fn author(&self) ->  Vec<String> {
        self.author.clone()
	}

    async fn genre(&self) ->  Vec<String> {
        self.genre.clone()
	}

    async fn status(&self) ->  Option<String> {
        self.status.clone()
	}

    async fn description(&self) ->  Option<String> {
        self.description.clone()
	}

    async fn path(&self) ->  String {
        self.path.clone()
	}

    async fn thumbnail_url(&self) ->  String {
        self.thumbnail_url.clone()
	}

    async fn last_read(&self) ->  Option<i32> {
        self.last_read
	}

    async fn last_page(&self) ->  Option<i32> {
        self.last_page
	}

    async fn is_favorite(&self) ->  bool {
        self.is_favorite
	}
}
