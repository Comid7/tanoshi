use std::rc::Rc;

use dominator::{Dom, html};

use crate::{app::App, common::Spinner};

pub struct Updates {
    spinner: Rc<Spinner>,
}

impl Updates {
    pub fn new() -> Rc<Self> {
        Rc::new(Updates{
            spinner: Spinner::new(),
        })
    }

    pub fn render_topbar() -> Dom {
        html!("div", {
            .class("w-full")
            .class("lg:w-auto")
            .class("px-2")
            .class("pb-2")
            .class("m-0")
            .class("lg:ml-48")
            .class("flex")
            .class("justify-center")
            .class("block")
            .class("fixed")
            .class("inset-x-0")
            .class("top-0")
            .class("z-50")
            .class("bg-accent")
            .class("dark:bg-gray-900")
            .class("border-b")
            .class("border-accent-darker")
            .class("dark:border-gray-800")
            .class("text-gray-50")
            .class("pt-safe-top")
            .children(&mut [
                html!("span", {
                    .class("text-gray-300")
                    .text("Updates")
                })
            ])
        })
    }

    pub fn render(updates: Rc<Self>, app: Rc<App>) -> Dom {
        html!{"div", {
            .children(&mut [
                Self::render_topbar()
            ])
        }}
    }
}