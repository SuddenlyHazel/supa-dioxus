#![allow(non_snake_case)]
mod components;

#[cfg(feature = "server")]
mod server;
pub mod client;

mod env;
mod pages;

use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use tracing::info;

use client::auth::AuthorizedClient;
use crate::pages::routes::{GuardContext, Router as AppRouter};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::initialize_default();

    server_only!({
        dotenv::dotenv().ok();
        info!("loaded env variables");
    });
    
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _guard_context = use_context_provider(|| Signal::new(GuardContext::default()));

    rsx! {
        head {
            script { src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2" }
        }
        Stylesheet { href: TAILWIND_CSS }
        AppRouter {
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(name = GetServerData, client=AuthorizedClient)]
async fn get_server_data() -> Result<String, ServerFnError> {
    use server::auth::SupabaseClient;
    let client: SupabaseClient = extract().await?;

    let resp = client
        .table("created_tasks")
        .select("*")
        .execute()
        .await?;

    info!("{:#?}", resp.text().await);
    Ok("Hello from the server!".to_string())
}
