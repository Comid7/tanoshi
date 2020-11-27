use std::cell::Cell;
use std::rc::Rc;

use dominator::{clone, events, html, link, text_signal, Dom, svg};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use futures::future::Future;

use super::Route;
use crate::query::fetch_manga_from_source;
pub struct Bottombar {
}

impl Bottombar {
    pub fn new() -> Rc<Self> {
        Rc::new(Bottombar {})
    }

    pub fn render() -> Dom {
        html!("div", {
            .class([
                "block",
                "fixed",
                "inset-x-0",
                "bottom-0",
                "lg:inset-y-0",
                "lg:left-0",
                "lg:w-48",
                "z-50",
                "border-t",
                "lg:border-r",
                "border-gray-300",
                "dark:border-gray-700",
                "safe-bottom",
                "bg-gray-100",
                "dark:bg-gray-800",
                "p-3",
                "flex", 
                "lg:flex-col", 
                "justify-evenly",
                "lg:justify-start",
                "pb-safe-bottom"
            ])
            .children(&mut [
                link!(Route::Library.url(), {
                    .class([
                        "flex",
                        "rounded",
                        "px-2",
                        "text-gray-900",
                        "dark:text-gray-50"
                    ])
                    .class_signal(["bg-gray-300", "dark:bg-gray-700"], Route::signal().map(|x| match x {
                        Route::Library => true,
                        _ => false,
                    }))
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
                            .class("my-2")
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z")
                                })
                            ])
                        }),
                        html!("span", {
                            .class([
                                "hidden",
                                "lg:block",
                                "my-auto",
                                "mx-2"
                            ])
                            .text("Library")
                        })
                    ])
                }),
                link!(Route::Catalogue(2).url(), {
                    .class([
                        "flex",
                        "rounded",
                        "px-2",
                        "text-gray-900",
                        "dark:text-gray-50"
                    ])
                    .class_signal(["bg-gray-300", "dark:bg-gray-700"], Route::signal().map(|x| match x {
                        Route::Catalogue(_) => true,
                        _ => false,
                    }))
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
                            .class("my-2")
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10")
                                })
                            ])
                        }),
                        html!("span", {
                            .class([
                                "hidden",
                                "lg:block",
                                "my-auto",
                                "mx-2"
                            ])
                            .text("Catalogue")
                        })
                    ])
                }),
                link!(Route::Updates.url(), {
                    .class([
                        "flex",
                        "rounded",
                        "px-2",
                        "text-gray-900",
                        "dark:text-gray-50"
                    ])
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
                            .class("my-2")
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9")
                                })
                            ])
                        }),
                        html!("span", {
                            .class([
                                "hidden",
                                "lg:block",
                                "my-auto",
                                "mx-2"
                            ])
                            .text("Updates")
                        })
                    ])
                }),
                link!(Route::Histories.url(), {
                    .class([
                        "flex",
                        "rounded",
                        "px-2",
                        "text-gray-900",
                        "dark:text-gray-50"
                    ])
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
                            .class("my-2")
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z")
                                })
                            ])
                        }),
                        html!("span", {
                            .class([
                                "hidden",
                                "lg:block",
                                "my-auto",
                                "mx-2"
                            ])
                            .text("History")
                        })
                    ])
                }),
                link!(Route::Catalogue(2).url(), {
                    .class([
                        "flex",
                        "rounded",
                        "px-2",
                        "text-gray-900",
                        "dark:text-gray-50"
                    ])
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
                            .class("my-2")
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z")
                                }),
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "1")
                                    .class("heroicon-ui")
                                    .attribute("d", "M15 12a3 3 0 11-6 0 3 3 0 016 0z")
                                })
                            ])
                        }),
                        html!("span", {
                            .class([
                                "hidden",
                                "lg:block",
                                "my-auto",
                                "mx-2"
                            ])
                            .text("Settings")
                        })    
                    ])
                }),
            ])
        })
    }
}
