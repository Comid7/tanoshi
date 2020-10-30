use futures::future::Future;
use futures_signals::signal_vec::MutableVec;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use web_sys::window;
use std::rc::Rc;

type NaiveDateTime = String;

use crate::common::Cover;

fn graphql_url() -> String {
    [
        window().unwrap().document().unwrap().location().unwrap().origin().unwrap(),
        "/graphql".to_string()
    ].join("")
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct BrowseSource;

pub async fn fetch_manga_from_source(source_id: i64) -> Result<Vec<Rc<Cover>>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = browse_source::Variables {
        source_id: Some(source_id),
        page: Some(1),
        sort_by: Some(browse_source::SortByParam::VIEWS),
        sort_order: Some(browse_source::SortOrderParam::DESC),
    };
    let request_body = BrowseSource::build_query(var);
    let res = client.post(&graphql_url()).json(&request_body).send().await?;
    let response_body: Response<browse_source::ResponseData> = res.json().await?;
    let list = response_body.data.unwrap_throw().browse_source;

    let covers = list.iter()
        .map(|item| Cover::new(item.id, item.title.clone(), item.cover_url.clone()))
        .collect();
    Ok(covers)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct BrowseFavorites;

pub async fn fetch_manga_from_favorite() -> Result<Vec<Rc<Cover>>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = browse_favorites::Variables{};
    let request_body = BrowseFavorites::build_query(var);
    let res = client.post(&graphql_url()).json(&request_body).send().await?;
    let response_body: Response<browse_favorites::ResponseData> = res.json().await?;
    let list = response_body.data.unwrap_throw().library;

    Ok(list.iter()
        .map(|item| Cover::new(item.id, item.title.clone(), item.cover_url.clone()))
        .collect())
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct FetchMangaDetail;

pub async fn fetch_manga_detail(id: i64) -> Result<fetch_manga_detail::FetchMangaDetailManga, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = fetch_manga_detail::Variables{
        id: Some(id),
    };
    let request_body = FetchMangaDetail::build_query(var);
    let res = client.post(&graphql_url()).json(&request_body).send().await?;
    let response_body: Response<fetch_manga_detail::ResponseData> = res.json().await?;
    let manga = response_body.data.unwrap_throw().manga.unwrap_throw();

    Ok(manga)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct FetchChapter;

pub async fn fetch_chapter(chapter_id: i64) -> Result<fetch_chapter::FetchChapterChapter, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = fetch_chapter::Variables{
        chapter_id: Some(chapter_id),
    };
    let request_body = FetchChapter::build_query(var);
    let res = client.post(&graphql_url()).json(&request_body).send().await?;
    let response_body: Response<fetch_chapter::ResponseData> = res.json().await?;
    let manga = response_body.data.unwrap_throw().chapter.unwrap_throw();

    Ok(manga)
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct AddToLibrary;

pub async fn add_to_library(manga_id: i64) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = add_to_library::Variables{
        manga_id: Some(manga_id),
    };
    let request_body = AddToLibrary::build_query(var);
    let _ = client.post(&graphql_url()).json(&request_body).send().await?;

    Ok(())
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct DeleteFromLibrary;

pub async fn delete_from_library(manga_id: i64) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = delete_from_library::Variables{
        manga_id: Some(manga_id),
    };
    let request_body = DeleteFromLibrary::build_query(var);
    let _ = client.post(&graphql_url()).json(&request_body).send().await?;

    Ok(())
}