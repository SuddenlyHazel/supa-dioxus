use dioxus::prelude::*;

use crate::pages::routes::Routes;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "max-w-lg mx-auto py-2",
            h1 {
              class: "text-3xl",
              "Home SupaDioxus"
            }
            Link { to: Routes::Protected {}, "Protected" }
        }
    }
}
