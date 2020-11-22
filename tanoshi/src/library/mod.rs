use crate::catalogue::{Chapter, Manga};
use crate::context::GlobalContext;
use crate::db::Db;
use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{
    Context, InputValueError, InputValueResult, Object, Result, Scalar, ScalarType, Value,
};
use chrono::{Local, NaiveDateTime};

mod library;
pub use library::{RecentChapter, RecentUpdate};

#[derive(Default)]
pub struct LibraryRoot;

#[Object]
impl LibraryRoot {
    async fn library(&self, ctx: &Context<'_>) -> Vec<Manga> {
        match ctx.data_unchecked::<GlobalContext>().db.get_library().await {
            Ok(mangas) => mangas,
            Err(_) => vec![],
        }
    }

    async fn recent_updates(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, RecentUpdate, EmptyFields, EmptyFields>> {
        let db = ctx.data_unchecked::<GlobalContext>().db.clone();
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let after_cursor = after
                    .and_then(|cursor: String| decode_cursor(&cursor).ok())
                    .unwrap_or(format!("{}#0", Local::now().timestamp()));
                let before_cursor = before
                    .and_then(|cursor: String| decode_cursor(&cursor).ok())
                    .unwrap_or(format!(
                        "{}#0",
                        NaiveDateTime::from_timestamp(0, 0).timestamp()
                    ));

                let edges = if let Some(first) = first {
                    db.get_first_recent_updates(&after_cursor, &before_cursor, first as i32)
                        .await
                } else if let Some(last) = last {
                    db.get_last_recent_updates(&after_cursor, &before_cursor, last as i32)
                        .await
                } else {
                    db.get_recent_updates(&after_cursor, &before_cursor).await
                };
                let edges = edges.unwrap_or(vec![]);

                let mut has_previous_page = false;
                let mut has_next_page = false;
                if edges.len() > 0 {
                    if let Some(e) = edges.first() {
                        has_previous_page = db.get_chapter_has_before_page(&format!("{}#{}", e.uploaded.timestamp(), e.chapter_id)).await;
                    }
                    if let Some(e) = edges.last() {
                        has_next_page = db.get_chapter_has_next_page(&format!("{}#{}", e.uploaded.timestamp(), e.chapter_id)).await;
                    }
                }

                let mut connection = Connection::new(has_previous_page, has_next_page);
                connection.append(
                    edges
                        .into_iter()
                        .map(|e| Edge::new(encode_cursor(e.uploaded.timestamp(), e.chapter_id), e)),
                );
                Ok(connection)
            },
        )
        .await
    }

    async fn recent_chapters(&self, ctx: &Context<'_>) -> Vec<RecentChapter> {
        match ctx
            .data_unchecked::<GlobalContext>()
            .db
            .get_recent_chapters()
            .await
        {
            Ok(chapters) => chapters,
            Err(_) => vec![],
        }
    }
}

fn decode_cursor(cursor: &String) -> std::result::Result<String, base64::DecodeError> {
    match base64::decode(cursor) {
        Ok(res) => Ok(String::from_utf8(res).unwrap_or("".to_string())),
        Err(err) => Err(err),
    }
}

fn encode_cursor(timestamp: i64, id: i64) -> String {
    base64::encode(format!("{}#{}", timestamp, id))
}

#[derive(Default)]
pub struct LibraryMutationRoot;

#[Object]
impl LibraryMutationRoot {
    async fn add_to_library(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "manga id")] manga_id: i64,
    ) -> Result<u64> {
        match ctx
            .data_unchecked::<GlobalContext>()
            .db
            .favorite_manga(manga_id, true)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("error add manga to library: {}", err).into()),
        }
    }

    async fn delete_from_library(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "manga id")] manga_id: i64,
    ) -> Result<u64> {
        match ctx
            .data_unchecked::<GlobalContext>()
            .db
            .favorite_manga(manga_id, false)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("error add manga to library: {}", err).into()),
        }
    }

    async fn update_page_read_at(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "page id")] page_id: i64,
    ) -> Result<u64> {
        match ctx
            .data_unchecked::<GlobalContext>()
            .db
            .update_page_read_at(page_id)
            .await
        {
            Ok(rows) => Ok(rows),
            Err(err) => Err(format!("error update page read_at: {}", err).into()),
        }
    }
}
