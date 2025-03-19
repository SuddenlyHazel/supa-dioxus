use crate::client::auth::signout;
use crate::components::button::Button;
use crate::pages::routes::{protected, GuardContext, Routes};
use dioxus::prelude::*;

#[component]
pub fn Protected() -> Element {
    protected(Routes::Login {}, Routes::Protected {});

    let logout = move |_| {
        spawn(async move {
            client! {
              let _ = signout().await;
                GuardContext::redirect_next_or_home();
            }
        });
    };

    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Protected SupaDioxus" }
            Button { text: "Signout", on_click: logout }
        }
    }
}
