use dioxus::prelude::*;

use crate::pages::routes::{protected, Routes};

#[component]
pub fn Protected() -> Element {
    protected(Routes::Login {}, Routes::Protected {});

    rsx! {
        div {
            class: "max-w-lg mx-auto py-2",
            h1 {
              class: "text-3xl",
              "Protected SupaDioxus"
          }
        }
    }
}
