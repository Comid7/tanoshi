use chrono::NaiveDateTime;
use async_graphql::{Context, Object, Result, Schema, Subscription, ID};

/// A type represent chapter, normalized across source
pub struct Chapter {
    pub id: i32,
    pub source: String,
    pub manga_id: i32,
    pub vol: Option<String>,
    pub no: Option<String>,
    pub title: Option<String>,
    pub path: String,
    pub read: Option<i32>,
    pub uploaded: chrono::NaiveDateTime,
}

#[Object]
impl Chapter {
    async fn id(&self) ->  i32{
        self.id
	}

    async fn source(&self) ->  String{
        self.source.clone()
	}

    async fn manga_id(&self) ->  i32{
        self.manga_id
	}

    async fn vol(&self) ->  Option<String>{
        self.vol.clone()
	}

    async fn no(&self) ->  Option<String>{
        self.no.clone()
	}

    async fn title(&self) ->  Option<String>{
        self.title.clone()
	}

    async fn path(&self) ->  String{
        self.path.clone()
	}

    async fn read(&self) ->  Option<i32>{
        self.read
	}

    async fn uploaded(&self) ->  NaiveDateTime{
        self.uploaded.clone()
    }
}