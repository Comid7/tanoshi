use dominator::traits::StaticEvent;
use wasm_bindgen::JsCast;

pub use dominator::events::*;

pub struct Error {
    event: web_sys::ErrorEvent,
}

impl StaticEvent for Error {
    const EVENT_TYPE: &'static str = "error";

    fn unchecked_from_event(event: web_sys::Event) -> Self {
        Self {
            event: event.unchecked_into(),
        }
    }
}