use dioxus::prelude::*;
use http::HeaderMap;

use crate::env;

pub struct SupabaseClient {
    client: postgrest::Postgrest,
    token: String,
}

impl SupabaseClient {
    pub fn table(&self, table: &str) -> postgrest::Builder {
        self.client.from(table).auth(self.token.clone())
    }
}

#[derive(Debug)]
pub struct NotFoundError;

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for NotFoundError {}

impl FromServerContext for SupabaseClient {
    type Rejection = NotFoundError;

    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn from_request<'life0, 'async_trait>(
        req: &'life0 dioxus::prelude::DioxusServerContext,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self, Self::Rejection>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            let headers = HeaderMap::from_request(req)
                .await
                .map_err(|_| NotFoundError)?;

            let bearer = headers
                .get(&format!("Authorization"))
                .ok_or(NotFoundError)?;

            // TODO this could fail in two ways
            let bearer = bearer.to_str().unwrap().split(" ").collect::<Vec<_>>()[1];

            let url = format!("{}/rest/v1", env::APP_PUBLIC_SUPABASE_URL);
            let client = postgrest::Postgrest::new(url)
                .insert_header("apikey", env::APP_PUBLIC_SUPABASE_ANON_KEY);

            Ok(SupabaseClient {
                client,
                token: bearer.into(),
            })
        })
    }
}
