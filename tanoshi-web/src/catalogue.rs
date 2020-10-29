use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use futures::future::Future;

use crate::{utils::AsyncLoader, common::{Cover, Spinner}};
use crate::query::fetch_manga_from_source;
pub struct Catalogue {
    pub source_id: i64,
    loader: AsyncLoader,
    spinner: Rc<Spinner>,
    cover_list: MutableVec<Rc<Cover>>,
}

impl Catalogue {
    pub fn new(source_id: i64) -> Rc<Self> {
        Rc::new(Catalogue {
            source_id,
            loader: AsyncLoader::new(),
            spinner: Spinner::new(),
            cover_list: MutableVec::new(),
        })
    }

    pub fn render_topbar(catalogue: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("px-2")
            .class("pb-2")
            .class("flex")
            .class("justify-between")
            .class("block")
            .class("fixed")
            .class("inset-x-0")
            .class("top-0")
            .class("z-50")
            .class("bg-accent")
            .class("border-b")
            .class("border-accent-darker")
            .class("text-white")
            .class("pt-safe-top")
            .children(&mut [
                html!("button", {
                    .text("Filter")
                }),
                html!("span", {
                    .class("text-gray-300")
                    .text("Catalogue")
                }),
                html!("button", {
                    .text("Refresh")
                })
            ])
        })
    }

    pub fn render_main(catalogue: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("grid")
            .class("grid-cols-3")
            .class("md:grid-cols-4")
            .class("lg:grid-cols-6")
            .class("xl:grid-cols-12")
            .class("gap-2")
            .class("pb-safe-bottom-scroll")
            .children_signal_vec(catalogue.cover_list.signal_vec_cloned().map(clone!(catalogue => move |cover| Cover::render(&cover))))
        })
    }

    pub fn fetch_mangas(catalogue: Rc<Self>) {
        catalogue.spinner.set_active(true);
        catalogue.loader.load(clone!(catalogue => async move {
            let covers = fetch_manga_from_source(catalogue.source_id).await.unwrap_throw();
            let mut cover_list = catalogue.cover_list.lock_mut();
            cover_list.replace_cloned(covers);
            catalogue.spinner.set_active(false);
        }));
    }

    pub fn render(catalogue: Rc<Self>) -> Dom {
        Self::fetch_mangas(catalogue.clone());
        html!("div", {
            .class("main")
            .children(&mut [
                Self::render_topbar(catalogue.clone()),
                Self::render_main(catalogue.clone()),
                Spinner::render(&catalogue.spinner)
            ])
        })
    }
}
