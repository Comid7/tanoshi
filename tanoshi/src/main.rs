extern crate argon2;
extern crate libloading as lib;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

// mod auth;
mod config;
mod extension;
// mod favorites;
// mod filters;
// mod handlers;
// mod history;
// mod update;
mod catalogue;
mod context;
mod db;
mod graphql;

use anyhow::Result;
use clap::Clap;

use crate::context::GlobalContext;
use crate::graphql::QueryRoot;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{BadRequest, Response};
use config::Config;
use std::convert::Infallible;
use std::sync::{Arc, RwLock};
use warp::http::{Response as HttpResponse, StatusCode};
use warp::{Filter, Rejection, Reply};

#[derive(Clap)]
#[clap(version = "0.14.0")]
struct Opts {
    /// Path to config file
    #[clap(long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();
    let config = Config::open(opts.config)?;

    let secret = config.secret;
    let mut extensions = extension::Extensions::new();
    if extensions
        .initialize(config.plugin_path.clone(), config.plugin_config)
        .is_err()
    {
        log::error!("error initialize plugin");
    }

    // let serve_static = filters::static_files::static_files();

    // let routes = api.or(serve_static).with(warp::log("manga"));

    let pool = db::establish_connection(config.database_path).await;

    let schema = Schema::build(
        QueryRoot::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .data(GlobalContext::new(pool, extensions))
    .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;

    return Ok(());
}
