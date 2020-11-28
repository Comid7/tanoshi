use wasm_bindgen::prelude::*;
use web_sys::Url;
use futures_signals::signal::{Signal, SignalExt};
use dominator::routing;

#[derive(Debug, Clone)]
pub enum SettingCategory {
    None,
    Reader,
    Source(i64),
}

#[derive(Debug)]
pub enum Route {
    Library,
    Catalogue(i64),
    Manga(i64),
    Chapter(i64),
    Updates,
    Histories,
    Settings(SettingCategory),
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
            } else if paths.len() == 1 {
                match paths[0] {
                    "updates" => Route::Updates,
                    "histories" => Route::Histories,
                    "settings" => Route::Settings(SettingCategory::None),
                    _ => Route::NotFound
                }
            } else if paths.len() == 2 {
                match paths[0] {
                    "catalogue" => {
                        match paths[1].parse::<i64>() {
                            Ok(id) => Route::Catalogue(id),
                            Err(_) => Route::NotFound,
                        }
                    }
                    "manga" => {
                        match paths[1].parse::<i64>() {
                            Ok(id) => Route::Manga(id),
                            Err(_) => Route::NotFound,
                        }
                    },
                    "chapter" => {
                        match paths[1].parse::<i64>() {
                            Ok(id) => Route::Chapter(id),
                            Err(_) => Route::NotFound,
                        }
                    }
                    "settings" => {
                        match paths[1] {
                            "reader" => Route::Settings(SettingCategory::Reader),
                            "sources" => Route::Settings(SettingCategory::Source(0)),
                            _ => Route::NotFound
                        }
                    }
                    _ => Route::NotFound,
                }
            }  else if paths.len() == 3 {
                match paths[0] {
                    
                    "settings" => {
                        match paths[1] {
                            "sources" => {
                                match paths[2].parse::<i64>() {
                                    Ok(source_id) => Route::Settings(SettingCategory::Source(source_id)),
                                    Err(_) => Route::NotFound,
                                }
                            }
                            _ => Route::NotFound
                        }
                    }
                    _ => Route::NotFound,
                }
            } else {
                Route::NotFound
            }
        })
    }

    pub fn url(&self) -> String {
        match self {
            Route::Library => "/".to_string(),
            Route::Catalogue(source_id) => ["/catalogue".to_string(), source_id.to_string()].join("/"),
            Route::Manga(manga_id) => ["/manga".to_string(), manga_id.to_string()].join("/"),
            Route::Chapter(chapter_id) => ["/chapter".to_string(), chapter_id.to_string()].join("/"),
            Route::Updates => "/updates".to_string(),
            Route::Histories => "/histories".to_string(),
            Route::Settings(SettingCategory::None) => "/settings".to_string(),
            Route::Settings(SettingCategory::Reader) => "/settings/reader".to_string(),
            Route::Settings(SettingCategory::Source(source_id)) => if *source_id > 0 {format!("/settings/sources/{}", source_id)} else {"/settings/sources".to_string()},
            Route::NotFound => "/notfound".to_string()
        }
    }
}