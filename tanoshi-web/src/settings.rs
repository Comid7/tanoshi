use dominator::{Dom, clone, html, link, routing};
use futures_signals::{signal_vec::MutableVec, signal_vec::SignalVecExt, signal::{Mutable, SignalExt}};
use web_sys::window;
use std::rc::Rc;
use crate::{app::App, common::SettingCategory, common::{Route, events}};
use crate::query::fetch_sources;

#[derive(Debug, Clone)]
pub struct Source {
    id: i64,
    name: String,
    version: String,
    icon: String,
}

pub struct Settings {
    page: Mutable<SettingCategory>,
    sources: MutableVec<Source>
}

impl Settings {
    pub fn new() -> Rc<Self> {
        return Rc::new(Settings{
            page: Mutable::new(SettingCategory::None),
            sources: MutableVec::new(),
        })
    }

    pub fn fetch_sources(settings: Rc<Self>, app: Rc<App>) {
        app.loader.load(clone!(settings => async move {
            match fetch_sources().await {
                Ok(result) => {
                    settings.sources.lock_mut().replace_cloned(result.iter().map(|s| Source {
                        id: s.id,
                        name: s.name.clone(),
                        version: s.version.clone(),
                        icon: s.icon.clone(),
                    }).collect()
                )},
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }));
    }

    pub fn render_topbar() -> Dom {
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
                "dark:bg-gray-900",
                "border-b",
                "border-accent-darker",
                "dark:border-gray-800",
                "text-gray-50",
                "pt-safe-top",
                "flex"
            ])
            .children(&mut [
                html!("button", {
                    .text("Close")
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
                    .class(["text-gray-300", "truncate", "mx-auto"])
                    .text("Settings")
                }),
                html!("span", {
                    .class("text-gray-300")
                    .text("Reset")
                })
            ])
        })
    }

    pub fn render_general_categories(settings: Rc<Self>) -> Dom {
        html!("div", {
            .class([
                "flex",
                "flex-col",
                "justify-start",
                "rounded-lg",
                "bg-gray-50",
                "dark:bg-gray-900",
                "divide-y",
                "divide-gray-200",
                "dark:divide-gray-800",
                "px-2"
            ])
            .children(&mut [
                link!(Route::Settings(SettingCategory::Reader).url(), {
                    .class([
                        "p-2",
                        "text-left"
                    ])
                    .text("Reader")
                }),
                link!(Route::Settings(SettingCategory::Source).url(), {
                    .class([
                        "p-2",
                        "text-left"
                    ])
                    .text("Source")
                })
            ])
        })
    }

    pub fn render_source_settings(settings: Rc<Self>) -> Dom {
        html!("div", {
            .children(&mut [
                html!("h1", {
                    .class([
                        "text-gray-900",
                        "dark:text-gray-100",
                        "hidden",
                        "lg:block",
                        "text-lg"
                    ])
                    .text("Source")
                }),
                html!("div", {
                    .class([
                        "rounded-lg",
                        "bg-gray-200",
                        "dark:bg-gray-800",
                        "divide-y",
                        "divide-gray-50",
                        "dark:divide-gray-900",
                        "px-2"
                    ])
                    .children_signal_vec(settings.sources.signal_vec_cloned().map(|x| 
                        html!("div", {
                            .class([
                                "p-2"
                            ])
                            .children(&mut [
                                html!("div", {
                                    .class("flex")
                                    .children(&mut [
                                        html!("img", {
                                            .class([
                                                "w-10",
                                                "h-10",
                                                "mr-2"
                                            ])
                                            .attribute("src", &["data:image/png;base64,", &x.icon].join(" "))
                                        }),
                                        html!("div", {
                                            .children(&mut [
                                                html!("div", {
                                                    .class([
                                                        "text-gray-900",
                                                        "dark:text-gray-50",
                                                    ])
                                                    .text(&x.name)
                                                }),
                                                html!("div", {
                                                    .class([
                                                        "text-gray-800",
                                                        "dark:text-gray-200",
                                                        "text-sm"
                                                    ])
                                                    .text(&x.version)
                                                })
                                            ])
                                        })
                                    ])
                                })
                            ])
                        })
                    ))
                })
            ])
        })
    }

    pub fn render(settings: Rc<Self>, app: Rc<App>, category: SettingCategory) -> Dom {
        settings.page.set(category.clone());
        match category {
            SettingCategory::None => {},
            SettingCategory::Reader => {},
            SettingCategory::Source => Self::fetch_sources(settings.clone(), app.clone())
        }
        html!("div", {
            .class([
                "main",
                "w-full",
                "lg:flex"
            ])
            .children(&mut [
                Self::render_topbar(),
                html!("div", {
                    .class([
                        "hidden",
                        "lg:block",
                        "w-0",
                        "lg:w-1/3",
                        "z-1"
                    ])
                }),
                html!("div", {
                    .class([
                        "w-full",
                        "lg:w-1/3",
                        "h-screen",
                        "static",
                        "lg:fixed",
                        "bg-gray-100",
                        "dark:bg-gray-800",
                        "text-gray-900",
                        "dark:text-gray-50",
                        "p-2",
                        "z-1",
                        "lg:z-2"
                    ])
                    .class_signal(["hidden", "lg:block"], settings.page.signal_cloned().map(|x| 
                        match x {
                            SettingCategory::None => false,
                            _ => true
                        }
                    ))
                    .class_signal(["block"], settings.page.signal_cloned().map(|x| 
                        match x {
                            SettingCategory::None => true,
                            _ => false
                        }
                    ))
                    .children(&mut [
                        html!("label", {
                            .class([
                                "text-sm",
                                "text-gray-800",
                                "dark:text-gray-100",
                                "p-2"
                            ])
                            .text("General")
                        }),
                        Self::render_general_categories(settings.clone())
                    ])
                }),
                html!("div", {
                    .class([
                        "w-full",
                        "lg:w-2/3",
                        "h-screen",
                        "z-2",
                        "lg:z-1",
                        "p-2"
                    ])
                    .child_signal(settings.page.signal_cloned().map(clone!(settings => move |x| 
                        match x {
                            SettingCategory::None => None,
                            SettingCategory::Reader => None,
                            SettingCategory::Source => Some(Self::render_source_settings(settings.clone()))
                        }
                    )))
                })
            ])
        })
    }
}