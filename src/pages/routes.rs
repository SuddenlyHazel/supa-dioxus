
use dioxus::prelude::{Router as DRouter, *};

use crate::client::auth::{self, get_user};

use super::{home::Home, login::Login, protected::Protected};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Routes {
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/protected")]
    Protected {},
}

/// Register the protected state of routes here
fn is_guarded(current: Routes) -> bool {
    // guard routes
    match current {
        Routes::Home {} => false,
        Routes::Login {} => false,
        Routes::Protected {} => true,
    }
}

#[component]
pub fn Router() -> Element {
    rsx! {
      DRouter::<Routes> {
        config: || {
          RouterConfig::default().on_update(|state| {
            if is_guarded(state.current()) {
              on_not_authorized(move |_| {
                GuardContext::set_next(state.current());
              });
            }
            None
          })
        }
      }
    }
}

#[derive(Default)]
pub struct GuardContext {
    next: Option<Routes>,
}

impl GuardContext {
    pub fn set_next(next: Routes) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    pub fn redirect_next_or_home() {
        let nav = navigator();
        let mut guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        if let Some(next) = next_maybe {
            nav.push(next);
        }
    }
}

fn on_not_authorized<F>(f: F)
where
    F: Fn(()) -> () + 'static,
{
    spawn(async move {
        let user = get_user().await;
        if let Err(_) = user {
            f(());
        }
    });
}

/// Declare a page view protected 
/// 
/// Automatically redirect users to login and back to the page on auth success
pub fn protected(redirect: Routes, next: Routes) {
    spawn(async move {
        let user = get_user().await;
        if let Err(_) = user {
            GuardContext::set_next(next);
            let nav = navigator();
            nav.replace(redirect);
            return;
        }
    });
}
