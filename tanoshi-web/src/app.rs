use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, svg, text_signal, Dom};
use futures::future::Future;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{utils::AsyncLoader, common::{Bottombar, Cover, Route, Spinner}};
use crate::library::Library;
use crate::catalogue::Catalogue;
use crate::manga::Manga;
use crate::reader::Reader;

pub struct App {
    pub library_page: Rc<Library>,
    pub catalogue_page: Rc<Catalogue>,
    pub manga_page: Rc<Manga>,
    pub reader_page: Rc<Reader>,
    pub spinner: Rc<Spinner>,
    pub loader: AsyncLoader,
}

impl App {
    pub fn new() -> Rc<Self> {
        Rc::new(App {
            library_page: Library::new(),
            catalogue_page: Catalogue::new(),
            manga_page: Manga::new(),
            reader_page: Reader::new(),
            spinner: Spinner::new(),
            loader: AsyncLoader::new(),
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("div", {
            .class("px-2")
            .children_signal_vec(Route::signal().map(move |x| { 
                match x {
                    Route::Library => vec![
                        Library::render(app.library_page.clone()),
                        Bottombar::render()
                    ],
                    Route::Catalogue(source_id) => vec![
                        Catalogue::render(app.catalogue_page.clone(), source_id),
                        Bottombar::render()
                    ],
                    Route::Manga(manga_id) => vec![
                        Manga::render(app.manga_page.clone(), app.spinner.clone(), manga_id),
                    ],
                    Route::Chapter(chapter_id) => vec![
                        Reader::render(app.clone(), chapter_id),
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
