use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use futures::future::Future;

use crate::query::fetch_manga_from_source;
pub struct Topbar {
}

impl Topbar {
    pub fn new() -> Rc<Self> {
        Rc::new(Topbar {})
    }

    pub fn render(topbar: Rc<Self>) -> Dom {
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
            .text("Tanoshi")
        })
    }
}
