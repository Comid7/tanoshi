use std::rc::Rc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::App;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cover {
    pub id: i64,
    pub title: String,
    pub cover_url: String,
}

impl Cover {
    pub fn new(id: i64, title: String, cover_url: String) -> Rc<Self> {
        Rc::new(Self {
            id,
            title,
            cover_url,
        })
    }

    pub fn render(cover: Rc<Self>) -> Dom {
        html!("div", {
            .class("cursor-pointer")
            .class("relative")
            .class("rounded-md")
            .class("pb-7/5")
            .children(&mut [
                html!("img", {
                    .class("absolute")
                    .class("w-full")
                    .class("h-full")
                    .class("object-cover")
                    .class("rounded-md")
                    .attribute("src", &cover.cover_url)
                    .children(&mut [
                        html!("div", {
                            .class("absolute")
                            .class("bottom-0")
                            .class("sm:text-sm")
                            .class("text-xs")
                            .class("bg-black")
                            .class("w-full")
                            .class("opacity-75")
                            .class("text-white")
                            .class("p-1")
                            .class("truncate")
                            .class("rounded-b-md")
                            .text(&cover.title)
                        })
                    ])
                })
            ])
        })
    }
}
