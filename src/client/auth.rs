use std::future::Future;

use dioxus::{prelude::{
    server_fn::{
        client::{browser::BrowserClient, Client},
        request::browser::BrowserRequest,
        response::browser::BrowserResponse,
    },
    ServerFnError,
}, signals::{GlobalSignal, Readable, Signal}};
use gloo::storage::Storage;
use supabase_js_rs::*;
use wasm_bindgen::{closure::Closure, JsValue};

use crate::env;
    
pub static CLIENT: GlobalSignal<supabase_js_rs::SupabaseClient> = Signal::global(|| create_client());

// auth funcs https://github.com/supabase/auth-js/blob/59ec9affa01c780fb18f668291fa7167a65c391d/src/GoTrueClient.ts#L1152
// https://github.com/supabase/auth-js/blob/59ec9affa01c780fb18f668291fa7167a65c391d/src/lib/types.ts#L325
// https://supabase.github.io/auth-js/v2/

// Custom web client to attach supabase bearer information onto requests
pub struct AuthorizedClient;
// https://github.com/leptos-rs/leptos/blob/97fd8ff6c46f742ce809398aa05161567b90a16b/examples/server_fns_axum/src/app.rs#L808-L852
impl<CustErr> Client<CustErr> for AuthorizedClient {
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, ServerFnError<CustErr>>> + Send {
        let res = gloo::storage::LocalStorage::get::<Session>(format!(
            "sb-{}-auth-token",
            env::APP_PUBLIC_ID
        ));
        if let Ok(session) = res {
            let headers = req.headers();
            headers.append("Authorization", &format!("Bearer {}", session.access_token));
        }
        BrowserClient::send(req)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub is_anonymous: Option<bool>,
    pub role: Option<String>,
    pub aud: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

// get_user types

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthError {}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserData {
    user: Option<User>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GetUserResponse {
    data: GetUserData,
    error: Option<AuthError>,
}

pub async fn get_user() -> Result<User, AuthError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.get_user(None).await {
        let res = serde_wasm_bindgen::from_value::<GetUserResponse>(res).unwrap();
        if let Some(error) = res.error {
            return Err(error);
        }

        return Ok(res.data.user.unwrap());
    }
    panic!("todo error")
}

pub async fn on_state_change() {
    let auth = CLIENT.read().auth();
    auth.on_auth_state_change(&Closure::new(move |event: JsValue, session: JsValue| {

    }));
}

// signin_with_password types
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponseData {
    user: Option<User>,
    session: Option<Session>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthResponseError {
    // Todo
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthResponsePassword {
    data: AuthResponseData,
    error: Option<AuthResponseError>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UserWithSession {
    pub user: User,
    session: Session,
}

pub async fn signin_with_password(
    credentials: Credentials,
) -> Result<UserWithSession, AuthResponseError> {
    let auth = CLIENT.read().auth();
    if let Ok(res) = auth.sign_in_with_password(credentials).await {
        let res = serde_wasm_bindgen::from_value::<AuthResponsePassword>(res).unwrap();

        if let Some(error) = res.error {
            return Err(error);
        }

        return Ok(UserWithSession {
            user: res.data.user.unwrap(),
            session: res.data.session.unwrap(),
        });
    }
    panic!("todo error")
}

fn create_client() -> SupabaseClient {
    supabase_js_rs::create_client(
        env::APP_PUBLIC_SUPABASE_URL,
        env::APP_PUBLIC_SUPABASE_ANON_KEY,
    )
}
