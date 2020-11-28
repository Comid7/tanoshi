use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, svg, text_signal, Dom};
use futures::future::Future;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{common::{Bottombar, Cover, Route, SettingCategory, Spinner}, histories::Histories, settings::Settings, updates::Updates, utils::AsyncLoader};
use crate::library::Library;
use crate::catalogue::Catalogue;
use crate::manga::Manga;
use crate::reader::Reader;

pub struct App {
    pub spinner: Rc<Spinner>,
    pub loader: AsyncLoader,
    pub settings_page: Rc<Settings>
}

impl App {
    pub fn new() -> Rc<Self> {
        Rc::new(App {
            spinner: Spinner::new(),
            loader: AsyncLoader::new(),
            settings_page: Settings::new(),
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("div", {
            .children_signal_vec(Route::signal().map(move |x| { 
                match x {
                    Route::Library => vec![
                        Library::render(Library::new()),
                        Bottombar::render()
                    ],
                    Route::Catalogue(source_id) => vec![
                        Catalogue::render(Catalogue::new(source_id)),
                        Bottombar::render()
                    ],
                    Route::Manga(manga_id) => vec![
                        Manga::render(Manga::new(manga_id), app.spinner.clone()),
                    ],
                    Route::Chapter(chapter_id) => vec![
                        Reader::render(Reader::new(chapter_id), app.clone()),
                    ],
                    Route::Updates => vec![
                        Updates::render(Updates::new(), app.clone()),
                        Bottombar::render()
                    ],
                    Route::Histories => vec![
                        Histories::render(Histories::new(), app.clone()),
                        Bottombar::render()
                    ],
                    Route::Settings(category) => vec![
                        Settings::render(app.settings_page.clone(), app.clone(), category),
                        Bottombar::render()
                    ],
                    Route::NotFound => vec![
                        Self::render(app.clone()),
                        Bottombar::render()
                    ]
                }
            }).to_signal_vec())
        })
    }
}
