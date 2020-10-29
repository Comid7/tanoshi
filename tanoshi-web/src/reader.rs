use crate::query::fetch_chapter;
use crate::app::App;
use crate::common::{events, Spinner};
use crate::utils::proxied_image_url;
use dominator::{html, svg, Dom, clone, link, routing};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVecExt};
use std::rc::Rc;
use web_sys::window;

pub struct Reader {
    pub manga_id: Mutable<i64>,
    pub manga_title: Mutable<String>,
    pub chapter_title: Mutable<String>,
    pub next_chapter: Mutable<Option<i64>>,
    pub prev_chapter: Mutable<Option<i64>>,
    pub pages: MutableVec<String>,
}

impl Reader {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            manga_id: Mutable::new(0),
            manga_title: Mutable::new("".to_string()),
            chapter_title: Mutable::new("".to_string()),
            next_chapter: Mutable::new(None),
            prev_chapter: Mutable::new(None),
            pages: MutableVec::new(),
        })
    }

    pub fn fetch_detail(app: Rc<App>, chapter_id: i64) {
        let reader = app.reader_page.clone();
        let spinner = app.spinner.clone();

        spinner.set_active(true);
        spinner.set_fullscreen(true);
        app.loader.load(clone!(reader => async move {
            match fetch_chapter(chapter_id).await {
                Ok(result) => {
                    reader.manga_id.set(result.manga.id);
                    reader.manga_title.set(result.manga.title);
                    reader.chapter_title.set(result.title);
                    reader.next_chapter.set(result.next);
                    reader.prev_chapter.set(result.prev);
                    reader.pages.lock_mut().replace_cloned(result.pages.iter().map(|page| page.url.clone()).collect());
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }

            spinner.set_active(false);
        }));
    }

    pub fn render_topbar(app: Rc<App>) -> Dom {
        let reader = app.reader_page.clone();
        html!("div", {
            .class([
                "flex",
                "justify-between",
                "items-center",
                "animated",
                "slideInDown",
                "faster",
                "block",
                "fixed",
                "inset-x-0",
                "top-0",
                "z-50",
                "bg-gray-900",
                "z-50",
                "content-end",
                "opacity-75",
                "pt-safe-top",
                "pb-2",
                "text-white"
            ])
            .children(&mut [
                html!("button", {
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("fill", "none")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .class(["w-6", "h-6"])
                            .children(&mut [
                                svg!("path", {
                                    .attribute("stroke-linecap", "round")
                                    .attribute("stroke-linejoin", "round")
                                    .attribute("stroke-width", "2")
                                    .attribute("d", "M6 18L18 6M6 6l12 12")
                                })
                            ])
                        })
                    ])
                    .event(|_: events::Click| {
                        let history = window().unwrap().history().unwrap();
                        if history.length().unwrap() > 1 {
                            history.back();
                        } else {
                            routing::go_to_url("/");
                        }
                    })
                }),
                html!("span", {
                    .class("truncate")
                    .text_signal(reader.chapter_title.signal_cloned().map(|t| t))
                }),
                html!("button", {
                    .children(&mut [
                        svg!("svg", {
                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                            .attribute("viewBox", "0 0 24 24")
                            .attribute("stroke", "currentColor")
                            .attribute("fill", "none")
                            .class("w-6")
                            .class("h-6")
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
                        })
                    ])
                })
            ])
        })
    }

    pub fn render_pages(app: Rc<App>) -> Dom {
        let reader = app.reader_page.clone();
        html!("div", {
            .children_signal_vec(reader.pages.signal_vec_cloned().map(clone!( app => move |page|
                html!("img", {
                    .attribute("src", &proxied_image_url(&page))
                    .event(clone!(app => move |_: events::Error| {
                        log::error!("error loading image");
                    }))
                })

            )))
        })
    }

    pub fn render(app: Rc<App>, chapter_id: i64) -> Dom {
        Self::fetch_detail(app.clone(), chapter_id);
        html!("div", {
            .children(&mut [
                Self::render_topbar(app.clone()),
                Self::render_pages(app.clone()),
                Spinner::render(&app.spinner)
            ])
        })
    }
}
