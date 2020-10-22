use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom, svg};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use futures::future::Future;

use crate::cover::Cover;
use crate::common::Bottombar;
use crate::query::fetch_manga_from_source;
pub struct App {
    cover_list: Vec<Rc<Cover>>,
}

impl App {
    pub fn new() -> Rc<Self> {
        Rc::new(App {
            cover_list: vec![],
        })
    }

    pub fn fetch_manga(covers: Vec<Rc<Cover>>) -> Rc<Self> {
        Rc::new(App {
            cover_list: covers,
        })
    }

    pub fn render_topbar(app: Rc<Self>) -> Dom {
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
                    .text("Library")
                }),
                html!("button", {
                    .text("Refresh")
                })
            ])
        })
    }

    pub fn render_library(app: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("mx-2")
            .class("grid")
            .class("grid-cols-3")
            .class("md:grid-cols-4")
            .class("lg:grid-cols-6")
            .class("xl:grid-cols-12")
            .class("gap-2")
            .class("pt-12")
            .children(app.cover_list.iter().map(clone!(app => move |cover| Cover::render(cover.clone(), app.clone()))))
        })
    }

    pub fn render(app: Rc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                Self::render_topbar(app.clone()),
                Self::render_library(app.clone()),
                Bottombar::render()
            ])
        })
    }
}
