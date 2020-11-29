use std::rc::Rc;

use crate::query::fetch_recent_updates;
use crate::utils::AsyncLoader;
use crate::{
    app::App,
    common::{Route, Spinner},
};
use dominator::{clone, events, html, link, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use futures_signals::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Clone)]
pub struct Entry {
    manga_id: i64,
    manga_title: String,
    cover_url: String,
    chapter_id: i64,
    chapter_title: String,
    uploaded: chrono::NaiveDateTime,
    cursor: String,
}

pub struct Updates {
    loader: AsyncLoader,
    spinner: Rc<Spinner>,
    entries: MutableVec<Entry>,
    is_entries_empty: Mutable<bool>,
}

impl Updates {
    pub fn new() -> Rc<Self> {
        Rc::new(Updates {
            spinner: Spinner::new(),
            loader: AsyncLoader::new(),
            entries: MutableVec::new(),
            is_entries_empty: Mutable::new(true),
        })
    }

    pub fn fetch_recent_chapters(updates: Rc<Self>) {
        updates.spinner.set_active(true);
        updates.loader.load(clone!(updates => async move {
            let cursor = updates.entries.lock_ref().last().map(|entry| entry.cursor.clone());
            match fetch_recent_updates(cursor).await {
                Ok(result) => {
                    for edge in result.edges.unwrap_throw() {
                        updates.entries.lock_mut().push_cloned(Entry{
                            manga_id: edge.as_ref().unwrap_throw().node.manga_id,
                            manga_title: edge.as_ref().unwrap_throw().node.manga_title.clone(),
                            cover_url: edge.as_ref().unwrap_throw().node.cover_url.clone(),
                            chapter_id: edge.as_ref().unwrap_throw().node.chapter_id,
                            chapter_title: edge.as_ref().unwrap_throw().node.chapter_title.clone(),
                            uploaded: chrono::NaiveDateTime::parse_from_str(&edge.as_ref().unwrap_throw().node.uploaded, "%Y-%m-%d %H:%M:%S").unwrap_throw(),
                            cursor: edge.as_ref().unwrap_throw().cursor.clone(),
                        })
                    }
                    updates.is_entries_empty.set(updates.entries.lock_ref().is_empty());
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
            updates.spinner.set_active(false);
        }));
    }

    pub fn render_topbar() -> Dom {
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
                html!("span", {
                    .class("text-gray-300")
                    .text("Updates")
                })
            ])
        })
    }

    pub fn render_main(updates: Rc<Self>) -> Dom {
        html!("div", {
            .class([
                "px-2",
                "lg:pr-2",
                "lg:pl-52",
            ])
            .children(&mut [
                html!("div", {
                    .children_signal_vec(updates.entries.signal_vec_cloned().map(|entry| {
                        link!(Route::Chapter(entry.chapter_id).url(), {
                            .class([
                                "flex",
                                "rounded",
                                "shadow",
                                "bg-gray-50",
                                "dark:bg-gray-800",
                                "p-2",
                                "m-2"
                            ])
                            .children(&mut [
                                html!("div", {
                                    .class([
                                        "pb-7/6",
                                        "mr-2"
                                    ])
                                    .children(&mut [
                                        html!("img", {
                                            .class([
                                                "w-16",
                                                "rounded",
                                                "object-cover"
                                            ])
                                            .attribute("src", &entry.cover_url)
                                        })
                                    ])
                                }),
                                html!("div", {
                                    .class(["flex-col"])
                                    .children(&mut [
                                        html!("h1", {
                                            .class([
                                                "text-gray-900",
                                                "dark:text-gray-50",
                                            ])
                                            .text(&entry.manga_title)
                                        }),
                                        html!("h2", {
                                            .class([
                                                "text-gray-900",
                                                "dark:text-gray-50",
                                            ])
                                            .text(&entry.chapter_title)
                                        }),
                                        html!("h2", {
                                            .class([
                                                "text-gray-900",
                                                "dark:text-gray-50",
                                            ])
                                            .text(&Self::calculate_days(entry.uploaded))
                                        })
                                    ])
                                })
                            ])
                        })
                    }))
                }),
                html!("div", {
                    .child_signal(updates.spinner.signal().map(clone!(updates => move |x| if x {
                        Some(Spinner::render(&updates.spinner))
                    } else {
                        Some(html!("button", {
                            .class([
                                "w-full",
                                "text-gray-900",
                                "dark:text-gray-50"
                            ])
                            .class_signal("disabled", updates.is_entries_empty.signal())
                            .text_signal(updates.is_entries_empty.signal().map(|x| 
                                if x {
                                    "No recent updates, favorite manga to see recent updates"
                                } else {
                                    "Load More"
                                }
                            ))
                            .event(clone!(updates => move |_: events::Click| {
                                Self::fetch_recent_chapters(updates.clone());
                            }))
                        }))
                    })))
                })
            ])
        })
    }

    fn calculate_days(at: chrono::NaiveDateTime) -> String {
        let timestamp = js_sys::Date::now();
        let secs: i64 = (timestamp / 1000.0).floor() as i64;
        let nanoes: u32 = (timestamp as u32 % 1000) * 1_000_000;
        let today = chrono::NaiveDateTime::from_timestamp(secs, nanoes);
        let days = today.date().signed_duration_since(at.date()).num_days();

        if days == 0 {
            "Today".to_string()
        } else if days == 1 {
            "Yesterday".to_string()
        } else if days > 1 && days <= 7 {
            format!("{} Days Ago", days)
        } else if days > 7 && days < 31 {
            format!("{} Weeks Ago", days / 7)
        } else {
            format!("{} Months Ago", days / 30)
        }
    }

    pub fn render(updates: Rc<Self>, app: Rc<App>) -> Dom {
        Self::fetch_recent_chapters(updates.clone());
        html! {"div", {
            .class([
                "main",
                "bg-gray-50",
                "dark:bg-gray-900"
            ])
            .children(&mut [
                Self::render_topbar(),
                Self::render_main(updates.clone())
            ])
        }}
    }
}
