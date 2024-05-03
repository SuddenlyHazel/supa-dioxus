use dioxus::prelude::*;

use crate::{
    components::{auth::Auth, layout::FlexStack},
    pages::routes::GuardContext,
};

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "max-w-lg mx-auto px-2 h-dvh place-content-center",
            FlexStack {
                class: "border-2 p-8 rounded-md border-orange-800 bg-gray-900 ",
                h1 {
                    class: "text-3xl",
                    "SupaDioxus"
                }
                Auth {
                    on_success: move |user| {
                      GuardContext::redirect_next_or_home();
                    }
                }
            }
        }
    }
}
