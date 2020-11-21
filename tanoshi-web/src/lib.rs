extern crate log;

use wasm_bindgen::prelude::*;

mod app;
mod catalogue;
mod common;
mod library;
mod manga;
mod query;
mod reader;
mod utils;
use app::App;

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    
    web_sys::window()
        .unwrap_throw()
        .document()
        .unwrap_throw()
        .body()
        .unwrap_throw()
        .class_list()
        .add_2("bg-gray-50", "dark:bg-gray-900")
        .unwrap_throw();
    dominator::append_dom(&dominator::body(), App::render(App::new()));

    Ok(())
}
