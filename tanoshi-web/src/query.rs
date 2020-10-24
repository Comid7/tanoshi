use futures::future::Future;
use futures_signals::signal_vec::MutableVec;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, spawn_local, JsFuture};
use std::rc::Rc;

use crate::common::Cover;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/query.graphql",
    response_derives = "Debug"
)]
pub struct BrowseSource;

pub async fn fetch_manga_from_source() -> Result<Vec<Rc<Cover>>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let var = browse_source::Variables {
        source_id: Some(2),
        page: Some(1),
        sort_by: Some(browse_source::SortByParam::VIEWS),
        sort_order: Some(browse_source::SortOrderParam::DESC),
    };
    let request_body = BrowseSource::build_query(var);
    let mut res = client.post("http://localhost:8000/graphql").json(&request_body).send().await?;
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
    let mut res = client.post("http://localhost:8000/graphql").json(&request_body).send().await?;
    let response_body: Response<browse_favorites::ResponseData> = res.json().await?;
    let list = response_body.data.unwrap_throw().library;

    Ok(list.iter()
        .map(|item| Cover::new(item.id, item.title.clone(), item.cover_url.clone()))
        .collect())
}

