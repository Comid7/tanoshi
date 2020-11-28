use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom};
use futures::future::{Future, ready};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::query::{
    browse_source::{SortByParam, SortOrderParam},
    fetch_manga_from_source,
};
use crate::{
    common::{Cover, Spinner},
    utils::AsyncLoader,
};
pub struct Catalogue {
    pub source_id: i64,
    keyword: Mutable<String>,
    page: Mutable<i64>,
    sort_by: Mutable<SortByParam>,
    sort_order: Mutable<SortOrderParam>,
    is_search: Mutable<bool>,
    loader: AsyncLoader,
    spinner: Rc<Spinner>,
    cover_list: MutableVec<Rc<Cover>>,
}

impl Catalogue {
    pub fn new(source_id: i64) -> Rc<Self> {
        Rc::new(Catalogue {
            source_id,
            keyword: Mutable::new("".to_string()),
            page: Mutable::new(1),
            sort_by: Mutable::new(SortByParam::VIEWS),
            sort_order: Mutable::new(SortOrderParam::DESC),
            is_search: Mutable::new(false),
            loader: AsyncLoader::new(),
            spinner: Spinner::new(),
            cover_list: MutableVec::new(),
        })
    }

    pub fn render_topbar(catalogue: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("lg:w-auto")
            .class("px-2")
            .class("pb-2")
            .class("m-0")
            .class("lg:ml-48")
            .class("flex")
            .class("justify-between")
            .class("block")
            .class("fixed")
            .class("inset-x-0")
            .class("top-0")
            .class("z-30")
            .class("bg-accent")
            .class("dark:bg-gray-900")
            .class("border-b")
            .class("border-accent-darker")
            .class("dark:border-gray-800")
            .class("text-gray-50")
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
                    .text("Search")
                    .event(clone!(catalogue => move |_: events::Click| {
                        catalogue.is_search.set_neq(true);
                    }))
                })
            ])
        })
    }

    pub fn render_main(catalogue: Rc<Self>) -> Dom {
        html!("div", {
            .class("px-2")
            .class("ml-0")
            .class("lg:ml-2")
            .class("lg:pr-2")
            .class("lg:pl-48")
            .class("pb-safe-bottom-scroll")
            .class("transition-all")
            .children(&mut [
                html!("div", {
                    .class("w-full")
                    .class("grid")
                    .class("grid-cols-3")
                    .class("md:grid-cols-4")
                    .class("lg:grid-cols-6")
                    .class("xl:grid-cols-12")
                    .class("gap-2")
                    .children_signal_vec(catalogue.cover_list.signal_vec_cloned().map(clone!(catalogue => move |cover| Cover::render(&cover))))
                }),
                html!("div", {
                    .child_signal(catalogue.spinner.signal().map(clone!(catalogue => move |x| if x {
                        Some(Spinner::render(&catalogue.spinner))
                    } else {
                        Some(html!("button", {
                            .class([
                                "w-full",
                                "text-gray-900",
                                "dark:text-gray-50"
                            ])
                            .text("Load More")
                            .event(clone!(catalogue => move |_: events::Click| {
                                catalogue.page.set(catalogue.page.get() + 1);
                                Self::fetch_mangas(catalogue.clone());
                            }))
                        }))
                    })))
                })
            ])
        })
    }

    pub fn render_search(catalogue: Rc<Self>) -> Dom {
        html!("div", {
            .class([
                "w-full",
                "mb-2",
                "ml-0",
                "lg:ml-2",
                "lg:pr-2",
                "lg:pl-48",
                "inline-flex",
                "transition-all",
            ])
            .visible_signal(catalogue.is_search.signal())
            .children(&mut [
                html!("input", {
                    .class([
                        "border",
                        "rounded",
                        "outline-none",
                        "w-full",
                        "mr-2",
                        "p-1"
                    ])
                    .attribute("placeholder", "Search")
                    .attribute("type", "text")
                    .property_signal("value", catalogue.keyword.signal_cloned())
                    .event(clone!(catalogue => move |event: events::Input| {
                        catalogue.keyword.set_neq(event.value().unwrap_throw());
                    }))
                    .event_preventable(clone!(catalogue => move |event: events::KeyDown| {
                        if event.key() == "Enter" {
                            event.prevent_default();
                            catalogue.cover_list.lock_mut().clear();
                            catalogue.page.set_neq(1);
                            Self::fetch_mangas(catalogue.clone());
                        }
                    }))
                }),
                html!("button", {
                    .text("Cancel")
                    .event(clone!(catalogue => move |_: events::Click| {
                        catalogue.is_search.set_neq(false);
                        if catalogue.keyword.get_cloned() != "" {
                            catalogue.keyword.set_neq("".to_string());
                            catalogue.cover_list.lock_mut().clear();
                            catalogue.page.set_neq(1);
                            Self::fetch_mangas(catalogue.clone());
                        }
                    }))
                })
            ])
        })
    }

    pub fn fetch_mangas(catalogue: Rc<Self>) {
        catalogue.spinner.set_active(true);
        catalogue.loader.load(clone!(catalogue => async move {
            let covers = fetch_manga_from_source(catalogue.source_id, catalogue.page.get(), Some(catalogue.keyword.get_cloned()), SortByParam::VIEWS, SortOrderParam::DESC).await.unwrap_throw();
            let mut cover_list = catalogue.cover_list.lock_mut();
            if catalogue.page.get() == 1 {
                cover_list.replace_cloned(covers);
            } else if catalogue.page.get() > 0 {
                for cover in covers.into_iter() {
                    cover_list.push_cloned(cover);
                }
            }
            catalogue.spinner.set_active(false);
        }));
    }

    pub fn render(catalogue: Rc<Self>) -> Dom {
        Self::fetch_mangas(catalogue.clone());
        html!("div", {
            .class([
                "main",
                "bg-gray-50",
                "dark:bg-gray-900"
            ])
            .children(&mut [
                Self::render_topbar(catalogue.clone()),
                Self::render_search(catalogue.clone()),
                Self::render_main(catalogue.clone()),
            ])
        })
    }
}
