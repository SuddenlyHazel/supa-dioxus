#![allow(non_snake_case)]
mod components;

#[cfg(feature = "server")]
mod server;

pub mod client;

mod env;
mod pages;

use client::auth::AuthorizedClient;
use dioxus::prelude::*;

use tracing::{info, Level};

use crate::pages::routes::{GuardContext, Router as AppRouter};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    server_only!({
        dotenv::dotenv().ok();
        info!("loaded env vars");
    });

    LaunchBuilder::fullstack().launch(App);
}

fn App() -> Element {
    let _guard_context = use_context_provider(|| Signal::new(GuardContext::default()));

    rsx! {
        head {
            script {
                src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2"
            }
        }
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
        .await
        .unwrap();

    info!("{:#?}", resp.text().await);
    Ok("Hello from the server!".to_string())
}
