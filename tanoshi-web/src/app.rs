use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, svg, text_signal, Dom};
use futures::future::Future;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::common::{Bottombar, Cover, Route, Topbar};
use crate::library::Library;
use crate::catalogue::Catalogue;
use crate::query::fetch_manga_from_source;
pub struct App {
    bottombar: Rc<Bottombar>,
    library_page: Rc<Library>,
    catalogue_page: Rc<Catalogue>,
}

impl App {
    pub fn new() -> Rc<Self> {
        Rc::new(App {
            bottombar: Bottombar::new(),
            library_page: Library::new(),
            catalogue_page: Catalogue::new(),
        })
    }

    pub fn render_main(app: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(Route::signal().map(move |x| {
                match x {
                    Route::Library => Some(Library::render(app.library_page.clone())),
                    Route::Catalogue(source_id) => Some(Catalogue::render(app.catalogue_page.clone())),
                    Route::NotFound => Some(Self::render(app.clone()))
                }
            }))
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                Self::render_main(app.clone()),
                Bottombar::render()
            ])
        })
    }
}
