use dioxus::prelude::{Router as DRouter, *};

use super::{home::Home, login::Login, protected::Protected, callback::Callback};
use crate::components::layout::NavLink;
use crate::{client::auth::get_user, components::layout::Nav};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Routes {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/protected")]
    Protected {},

    #[route("/callback")]
    Callback {},
}

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        header { class: "absolute inset-x-0 top-0 z-50",
            Nav {
                nav_items: vec![
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Protected {}), text :
                        "Protected" }
                    },
                ],
            }
        }
        div { class: "relative isolate pt-16", Outlet::<Routes> {} }
    }
}

/// Register the protected state of routes here
fn is_guarded(current: Routes) -> bool {
    // guard routes
    match current {
        Routes::Home {} => false,
        Routes::Login {} => false,
        Routes::Protected {} => true,
        Routes::Callback { .. } => false,
    }
}

#[component]
pub fn Router() -> Element {
    rsx! {
        DRouter::<Routes> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        if is_guarded(state.current()) {
                            on_not_authorized(move |_| {
                                GuardContext::set_next(state.current());
                            });
                        }
                        None
                    })
            },
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
        } else {
            nav.push(Routes::Home {});
        }
    }
}

fn on_not_authorized<F>(f: F)
where
    F: Fn(()) + 'static,
{
    spawn(async move {
        let user = get_user().await;
        if user.is_err() {
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
        if user.is_err() {
            GuardContext::set_next(next);
            let nav = navigator();
            nav.replace(redirect);
        }
    });
}
