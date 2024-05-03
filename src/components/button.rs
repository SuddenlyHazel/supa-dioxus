#![allow(non_snake_case)]

use std::ops::Range;

use dioxus::prelude::*;

#[component]
pub fn Button(text: String, class: Option<String>, on_click: EventHandler<MouseEvent>) -> Element {
    let class = class.unwrap_or_else(|| "".to_string());
    rsx! {
      button {
        onclick: move |evt| on_click.call(evt),
        class: "{class} btn-primary",
        "{text}"
      }
    }
}

#[component]
pub fn ButtonAlt(text: String, class: Option<String>, on_click: EventHandler<MouseEvent>, range: Option<Range<i64>>) -> Element {
    let class = class.unwrap_or_else(|| "".to_string());
    
    rsx! {
      button {
        onclick: move |evt| on_click.call(evt),
        class: "{class} btn-alt",
        "{text}"
      }
    }
}
