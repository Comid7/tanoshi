use crate::query::fetch_manga_detail;
use crate::utils::{proxied_image_url, AsyncLoader};
use crate::{
    app::App,
    common::{Route, Spinner},
};
use chrono::NaiveDateTime;
use dominator::{clone, events, html, link, svg, Dom};
use futures_signals::signal::SignalExt;
use futures_signals::{
    signal::Mutable,
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[derive(Clone)]
struct Chapter {
    pub id: i64,
    pub title: String,
    pub uploaded: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

pub struct Manga {
    title: Mutable<Option<String>>,
    author: MutableVec<String>,
    genre: MutableVec<String>,
    cover_url: Mutable<Option<String>>,
    description: Mutable<Option<String>>,
    status: Mutable<Option<String>>,
    chapters: MutableVec<Chapter>,
    loader: AsyncLoader,
}

impl Manga {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            title: Mutable::new(None),
            author: MutableVec::new(),
            genre: MutableVec::new(),
            cover_url: Mutable::new(None),
            description: Mutable::new(None),
            status: Mutable::new(None),
            chapters: MutableVec::new(),
            loader: AsyncLoader::new(),
        })
    }

    pub fn fetch_detail(manga: Rc<Self>, spinner: Rc<Spinner>, id: i64) {
        spinner.set_active(true);
        spinner.set_fullscreen(true);
        manga.loader.load(clone!(manga, spinner => async move {
            match fetch_manga_detail(id).await {
                Ok(result) => {
                    manga.title.lock_mut().replace(result.title);
                    manga.author.lock_mut().replace_cloned(result.author);
                    manga.genre.lock_mut().replace_cloned(result.genre);
                    manga.cover_url.lock_mut().replace(result.cover_url);
                    manga.description.lock_mut().replace(result.description.unwrap_throw());
                    manga.status.lock_mut().replace(result.status.unwrap_throw());
                    manga.chapters.lock_mut().replace_cloned(result.chapters.iter().map(|chapter| Chapter{
                        id: chapter.id,
                        title: chapter.title.clone(),
                        uploaded: NaiveDateTime::parse_from_str(&chapter.uploaded, "%Y-%m-%d %H:%M:%S").unwrap_throw(),
                        read_at: chapter.read_at.as_ref().map(|time| NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap_throw()),
                    }).collect());
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }

            spinner.set_active(false);
        }));
    }

    pub fn render_topbar(manga: &Self) -> Dom {
        html!("div", {
            .class([
                "w-full",
                "px-2",
                "pb-2",
                "flex",
                "justify-center",
                "block",
                "fixed",
                "inset-x-0",
                "top-0",
                "z-50",
                "bg-accent",
                "border-b",
                "border-accent-darker",
                "text-white",
                "pt-safe-top",
            ])
            .children(&mut [
                html!("span", {
                    .class(["text-gray-300", "truncate"])
                    .text_signal(manga.title.signal_cloned().map(|x| x.unwrap_or("".to_string())))
                }),
            ])
        })
    }

    pub fn render_header(manga: &Self) -> Dom {
        html!("div", {
            .attribute("id", "detail")
            .class([
                "flex", 
                "flex-col", 
                "justify-center", 
                "border", 
                "border-gray-300", 
                "dark:border-gray-700", 
                "bg-white", 
                "dark:bg-gray-900", 
                "p-2", 
                "mb-2", 
                "rounded",
                "shadow",
            ])
            .children(&mut [
                html!("div", {
                    .class("flex")
                    .children(&mut [
                        html!("div", {
                            .class(["pb-7/6", "mr-2"])
                            .children(&mut [
                                html!("img", {
                                    .class(["w-32", "rounded", "object-cover"])
                                    .attribute_signal("src", manga.cover_url.signal_cloned().map(|x| proxied_image_url(&x.unwrap_or("".to_string()))))
                                })
                            ])
                        }),
                        html!("div", {
                            .class(["flex", "flex-col"])
                            .children(&mut [
                                html!("span", {
                                    .class(["md:text-xl", "sm:text-base", "font-bold", "text-gray-900", "dark:text-gray-300"])
                                    .text("Author")
                                }),
                                html!("div", {
                                    .children_signal_vec(manga.author.signal_vec_cloned().map(|x| {
                                        html!("span", {
                                            .class(["md:text-xl", "sm:text-sm", "text-gray-900", "dark:text-gray-300", "mr-2"])
                                            .text(&x)
                                        })
                                    }))
                                }),
                                html!("span", {
                                    .class(["md:text-xl", "sm:text-base", "font-bold", "text-gray-900", "dark:text-gray-300"])
                                    .text("Status")
                                }),
                                html!("div", {
                                    .children(&mut [
                                        html!("span", {
                                            .class(["md:text-xl", "sm:text-sm", "text-gray-900", "dark:text-gray-300", "mr-2"])
                                            .text_signal(manga.status.signal_cloned().map(|x| x.unwrap_or("".to_string())))
                                        })
                                    ])
                                })
                            ])
                        })
                    ])
                })
            ])
        })
    }

    pub fn render_description(manga: &Self) -> Dom {
        html!("div", {
            .attribute("id", "description")
            .class([
                "flex", 
                "flex-col", 
                "justify-center", 
                "border", 
                "border-gray-300", 
                "dark:border-gray-700", 
                "bg-white",
                "dark:bg-gray-900", 
                "p-2", 
                "mb-2",
                "rounded",
                "shadow",
            ])
            .children(&mut [
                html!("span", {
                    .class(["md:text-xl", "sm:text-base", "font-bold", "text-gray-900", "dark:text-gray-300"])
                    .text("Description")
                }),
                html!("p", {
                    .class(["break-normal", "md:text-base", "sm:text-xs", "text-gray-900", "dark:text-gray-300"])
                    .text_signal(manga.description.signal_cloned().map(|x| x.unwrap_or("".to_string())))
                }),
                html!("div", {
                    .class(["w-full", "flex", "flex-wrap"])
                    .children_signal_vec(manga.genre.signal_vec_cloned().map(|x| {
                        html!("span", {
                            .class(["md:text-base", "sm:text-xs", "text-gray-900", "dark:text-gray-300", "mr-2", "rounded-full", "border", "border-gray-900", "dark:border-gray-300", "px-2", "mt-2"])
                            .text(&x)
                        })
                    }))
                })
            ])
        })
    }

    pub fn render_chapters(manga: &Self) -> Dom {
        html!("div", {
            .attribute("id", "description")
            .class([
                "flex", 
                "justify-center", 
                "border", 
                "border-gray-300", 
                "dark:border-gray-700", 
                "bg-white", 
                "dark:bg-gray-900", 
                "p-2", 
                "mb-2", 
                "w-full", 
                "lg:w-2/3", 
                "rounded",
                "shadow",
            ])
            .children(&mut [
                html!("div", {
                    .class(["flex", "flex-col", "w-full"])
                    .children(&mut [
                        html!("span", {
                            .class(["md:text-xl", "sm:text-base", "font-bold", "text-gray-900", "dark:text-gray-300"])
                            .text("Chapters")
                        }),
                    ])
                    .children_signal_vec(manga.chapters.signal_vec_cloned().map(|chapter| {
                        link!(Route::Chapter(chapter.id).url(), {
                            .class(["flex", "inline-flex", "hover:bg-gray-200", "dark:hover:bg-gray-700", "mt-2", "mr-2"])
                            .children(&mut [
                                html!("div", {
                                    .class(["flex", "justify-between", "items-center", "w-full", "text-gray-900", "dark:text-gray-300"])
                                    .children(&mut [
                                        html!("div", {
                                            .class(["flex", "flex-col"])
                                            .children(&mut [
                                                html!("span", {
                                                    .class(["text-md", "font-semibold"])
                                                    .text(&chapter.title)
                                                }),
                                                html!("span", {
                                                    .class("text-sm")
                                                    .text(&chapter.uploaded.date().to_string())
                                                })
                                            ])
                                        }),
                                        svg!("svg", {
                                            .attribute("xmlns", "http://www.w3.org/2000/svg")
                                            .attribute("fill", "none")
                                            .attribute("viewBox", "0 0 24 24")
                                            .attribute("stroke", "currentColor")
                                            .class("w-6")
                                            .class("h-6")
                                            .children(&mut [
                                                svg!("path", {
                                                    .attribute("stroke-linecap", "round")
                                                    .attribute("stroke-linejoin", "round")
                                                    .attribute("stroke-width", "2")
                                                    .attribute("d", "M9 5l7 7-7 7")
                                                })
                                            ])
                                        })
                                    ])
                                })
                            ])
                        })
                    }))
                })
            ])
        })
    }

    pub fn render(manga_page: Rc<Self>, spinner: Rc<Spinner>, id: i64) -> Dom {
        Self::fetch_detail(manga_page.clone(), spinner.clone(), id);
        html!("div", {
            .class(["pt-12", "w-full", "mx-auto", "lg:mx-2", "flex", "flex-col", "lg:flex-row"])
            .children(&mut [
                html!("div", {
                    .class(["w-full", "lg:w-1/3", "mr-2"])
                    .children(&mut [
                        Self::render_topbar(&manga_page),
                        Self::render_header(&manga_page),
                        Self::render_description(&manga_page),
                    ])
                }),
                Self::render_chapters(&manga_page),
                Spinner::render(&spinner)
            ])
        })
    }
}
