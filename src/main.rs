#![allow(non_snake_case)]
mod components;

#[cfg(feature = "server")]
mod server;

pub mod client;

mod env;
mod pages;

const _TAILWIND_URL: &str = manganis::mg!(file("assets/tailwind.css"));

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

    let builder = LaunchBuilder::fullstack();
    let mut config = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    {
        config = config.addr(std::net::SocketAddr::from(([0, 0, 0, 0], 8080)));
    }

    builder.with_cfg(config).launch(App);
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
