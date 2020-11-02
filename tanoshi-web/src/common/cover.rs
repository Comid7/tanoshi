use std::rc::Rc;

use dominator::{Dom, clone, events, html, link, with_node};
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, SignalExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::App;
use crate::utils::proxied_image_url;

use super::Route;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cover {
    pub id: i64,
    pub title: String,
    pub cover_url: String,
}

impl Cover {
    pub fn new(id: i64, title: String, cover_url: String) -> Rc<Self> {
        let cover_url = proxied_image_url(&cover_url);
        Rc::new(Self {
            id,
            title,
            cover_url,
        })
    }

    pub fn render(cover: &Self) -> Dom {
        link!(Route::Manga(cover.id).url(), {
            .class("cursor-pointer")
            .class("relative")
            .class("rounded-md")
            .class("pb-7/5")
            .class("shadow")
            .children(&mut [
                html!("img", {
                    .class("absolute")
                    .class("w-full")
                    .class("h-full")
                    .class("object-cover")
                    .class("rounded-md")
                    .attribute("src", &cover.cover_url)
                }),
                html!("span", {
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
    }
}
