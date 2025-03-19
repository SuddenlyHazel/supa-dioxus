use crate::client::auth::set_session;
use crate::pages::routes::Routes::Protected;
use dioxus::prelude::*;
use std::collections::HashMap;
use web_sys::window;

#[component]
pub fn Callback() -> Element {
    spawn(async move {
        client! {
            let hash = window().unwrap().location().hash().unwrap();
            let params_parsed: HashMap<String, String> = serde_urlencoded::from_str(hash
                .strip_prefix("#").unwrap())
            .unwrap();
            if let (Some(access_token), Some(refresh_token)) = (params_parsed.get("access_token"), 
                params_parsed.get("refresh_token")) {
                set_session(
                    access_token.to_owned(),
                    refresh_token.to_owned(),
                ).await;
                let nav = navigator();
                nav.replace(Protected {});
            }
        }
    });
    rsx! {}
}
