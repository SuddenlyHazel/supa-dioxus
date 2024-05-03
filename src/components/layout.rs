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
        class: {class},
        {children}
      }
    }
}

#[component]
pub fn Modal(title: String, body_children: Element, footer_children: Element, is_open: bool, on_close: EventHandler<()>) -> Element {
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
