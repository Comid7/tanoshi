extern crate log;

use wasm_bindgen::prelude::*;

mod query;
mod common;
mod catalogue;
mod library;
mod utils;
mod app;
use app::App;


#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    // let covers = query::fetch_manga_from_source()
    // .await
    // .unwrap()
    // .iter()
    // .map(|cover| crate::cover::Cover::new(cover.id, cover.title.clone(), cover.cover_url.clone()))
    // .collect();
    dominator::append_dom(&dominator::get_id("app"), App::render(App::new()));

    Ok(())
}
