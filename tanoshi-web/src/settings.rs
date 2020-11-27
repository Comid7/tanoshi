use dominator::{Dom, html};
use std::rc::Rc;
use crate::app::App;

pub struct Settings {

}

impl Settings {
    pub fn new() -> Rc<Self> {
        return Rc::new(Settings{

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
                    .text("Settings")
                })
            ])
        })
    }
    
    pub fn render_main() -> Dom {
        html!("div", {
            
        })    
    }

    pub fn render(settings: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("main")
            .children(&mut [
                Self::render_topbar(),
                Self::render_main()
            ])
        })
    }
}