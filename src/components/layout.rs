#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn FlexStack(class: Option<String>, children: Element) -> Element {
    let class = format!(
        "flex flex-col space-y-4 {}",
        class.unwrap_or_else(|| "".to_string())
    );
    rsx! {
      div {
        class: class,
        {children}
      }
    }
}

#[component]
pub fn Modal(
    title: String,
    body_children: Element,
    footer_children: Element,
    is_open: bool,
    on_close: EventHandler<()>,
) -> Element {
    let modal = rsx! {
      div {
        class: "fixed inset-0 z-40 bg-gray-900 bg-opacity-50 dark:bg-opacity-80",
      }
      div {
        class: "fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex justify-center items-center",
      }
      div {
        class: "fixed top-0 start-0 end-0 h-modal md:inset-0 md:h-full z-50 w-full p-4 flex justify-center items-center",
        div {
          class: "flex relative max-w-2xl w-full max-h-full",
          div {
            class: "bg-gray-800 text-gray-400 rounded-lg  border-gray-700 divide-gray-700 shadow-md relative flex flex-col mx-auto w-full divide-y",
            // Title
            div {
              class : "bg-gray-800 text-gray-400 border-gray-700 divide-gray-700 flex justify-between items-center p-4 md:p-5 rounded-t-lg",
              h3 {
                class: "text-xl font-semibold p-0",
                "{title}"
              }
              button {
                onclick: move |_| on_close.call(()),
                "close"
              }
            }
            // Body
            div {
              class : "p-4 md:p-5 space-y-4 flex-1 overflow-y-auto overscroll-contain",
              {body_children}
            }
            // Footer
            div {
              class: "bg-gray-800 text-gray-400 border-gray-700 divide-gray-700 flex items-center p-4 md:p-5 space-x-3 rtl:space-x-reverse rounded-b-lg",
              {footer_children}
            }
          }
        }
      }
    };

    if is_open {
        modal
    } else {
        rsx!()
    }
}

#[component]
pub fn List(class: Option<String>, list_item_class: Option<String>, children: Vec<(Element, String)>) -> Element {
    let class = format!(
        "divide-y divide-gray-200 {}",
        class.unwrap_or_else(|| "".to_string())
    );
    
    let list_item_class = format!("py-4 {}", list_item_class.unwrap_or_else(|| "".to_string()));
    rsx! {
      ul {
        role: "list",
        class: class,
        for child in children {
          li {
            key: "{child.1}",
            class: list_item_class.clone(),
            {child.0}
          }
        }
      }
    }
}

#[component]
pub fn NavLink(to : IntoRoutable, text : String) -> Element {
  rsx! {
    Link {
      active_class: "text-white font-bold underline decoration-white",
      class: "text-gray-300 hover:bg-gray-700 hover:text-white rounded-md px-3 py-2 text-sm font-medium",
      to: to,
      {text}
    }
  }
}

#[component]
pub fn Nav(nav_items : Vec<Element>) -> Element {
    rsx! {
      nav {
        class: "bg-slate-900",
        div {
          class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
          div {
            class: "relative flex h-16 items-center justify-between",
            div {
              class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
            }
            //       <div class="flex flex-1 items-center justify-center sm:items-stretch sm:justify-start">
            div {
                class: "flex flex-1 items-center justify-center sm:items-stretch sm:justify-start",
                div {
                  class: "flex flex-shrink-0 items-center",
                  span {
                    class: "font-bold text-2xl",
                    "SupaDioxus"
                  }
                }
                div {
                  class: "sm:ml-6 sm:block",
                  div {
                    class: "flex space-x-4",
                    for nav in nav_items {
                      {nav}
                    }
                  }
                }
            }
          }
        }
      }
    }
}

