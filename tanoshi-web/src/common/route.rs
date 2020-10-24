use wasm_bindgen::prelude::*;
use web_sys::Url;
use futures_signals::signal::{Signal, SignalExt};
use dominator::routing;

#[derive(Debug)]
pub enum Route {
    Library,
    Catalogue(i32),
    NotFound,
}

impl Route {
    pub fn signal() -> impl Signal<Item = Self> {
        routing::url()
        .signal_ref(|url| Url::new(&url).unwrap_throw())
        .map(|url| {
            let pathname = url.pathname();
            let mut paths = pathname.split("/").collect::<Vec<&str>>();
            paths.retain(|path| *path != "");
            if paths.len() == 0 {
                Route::Library
            } else if paths.len() == 2 {
                Route::Catalogue(2)
            } else {
                Route::NotFound
            }
        })
    }

    pub fn url(&self) -> String {
        match self {
            Route::Library => "/".to_string(),
            Route::Catalogue(source_id) => format!("/catalogue/{}", source_id),
            Route::NotFound => "/notfound".to_string()
        }
    }
}