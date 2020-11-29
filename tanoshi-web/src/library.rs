use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::common::{Cover, Spinner};
use crate::query::fetch_manga_from_favorite;
use crate::utils::AsyncLoader;

pub struct Library {
    loader: AsyncLoader,
    spinner: Rc<Spinner>,
    cover_list: MutableVec<Rc<Cover>>,
}

impl Library {
    pub fn new() -> Rc<Self> {
        Rc::new(Library {
            loader: AsyncLoader::new(),
            spinner: Spinner::new(),
            cover_list: MutableVec::new(),
        })
    }

    pub fn render_topbar(spinner: Rc<Spinner>) -> Dom {
        html!("div", {
            .class([
                "w-full",
				"px-2",
				"pb-2",
				"flex",
				"justify-between",
				"fixed",
				"inset-x-0",
				"top-0",
				"z-50",
				"bg-accent",
				"dark:bg-gray-900",
				"border-b",
				"border-accent-darker",
				"dark:border-gray-800",
				"text-gray-50",
				"pt-safe-top"
            ])
            .children(&mut [
                html!("button", {
                    .text("Filter")
                }),
                html!("span", {
                    .class("text-gray-300")
                    .text("Library")
                }),
                html!("button", {
                    .text("Refresh")
                    .event(clone!(spinner => move |_: events::Click| {
                        spinner.set_active(true);
                    }))
                })
            ])
        })
    }

    pub fn render_main(library: &Self) -> Dom {
        html!("div", {
            .class(["w-full",
                    "grid",
                    "grid-cols-3",
                    "md:grid-cols-4",
                    "lg:grid-cols-6",
                    "xl:grid-cols-12",
                    "gap-2",
                    "px-2",
                    "lg:pr-2",
                    "lg:pl-52",
                    "pb-safe-bottom-scroll"
            ])
            .children_signal_vec(library.cover_list.signal_vec_cloned().map(clone!(library => move |cover| Cover::render(&cover))))
        })
    }

    pub fn render(library: Rc<Self>) -> Dom {
        library.spinner.set_active(true);
        library.loader.load(clone!(library => async move {
            let covers = fetch_manga_from_favorite().await.unwrap_throw();
            let mut cover_list = library.cover_list.lock_mut();
            cover_list.replace_cloned(covers);
            library.spinner.set_active(false);
        }));
        html!("div", {
            .class([
                "main",
                "bg-gray-50",
                "dark:bg-gray-900"
            ])
            .children(&mut [
                Self::render_topbar(library.spinner.clone()),
                Self::render_main(&library),
                Spinner::render(&library.spinner)
            ])
        })
    }
}
