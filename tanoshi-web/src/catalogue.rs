use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use futures::future::Future;

use crate::cover::Cover;
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

    pub fn render(app: Rc<Self>) -> Dom {
        html!("div", {
            .class("w-full")
            .class("xl:w-1/2")
            .class("mx-auto")
            .class("grid")
            .class("grid-cols-3")
            .class("md:grid-cols-4")
            .class("lg:grid-cols-6")
            .class("xl:grid-cols-8")
            .class("gap-2")
            .class("pt-12")
            .children(app.cover_list.iter().map(clone!(app => move |cover| Cover::render(cover.clone(), app.clone()))))
        })
    }
}
