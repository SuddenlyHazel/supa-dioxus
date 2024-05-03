#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn TextInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
      input {
        value: "{i_value}",
        class: "input-primary",
        placeholder: "{i_placeholder}",
        oninput: move |event| on_input.call(event)
      }
    }
}

#[component]
pub fn PasswordInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
      input {
        r#type: "password",
        value: "{i_value}",
        class: "input-primary",
        placeholder: "{i_placeholder}",
        oninput: move |event| on_input.call(event)
      }
    }
}

#[component]
pub fn NumberInput(
    i_value: String,
    i_placeholder: Option<String>,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
    i_min: Option<String>,
    i_max: Option<String>,
    i_step: Option<String>,
) -> Element {
    let i_placeholder = i_placeholder.unwrap_or_else(|| "".to_string());
    rsx! {
      input {
        r#type: "number",
        value: "{i_value}",
        class: "input-primary",
        placeholder: "{i_placeholder}",
        oninput: move |event| on_input.call(event),
        min: i_min.unwrap_or_else(|| "".to_string()),
        max: i_max.unwrap_or_else(|| "".to_string()),
        step: i_step.unwrap_or_else(|| "".to_string()),
      }
    }
}

#[component]
pub fn DateInput(
    i_value: String,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
) -> Element {
    rsx! {
      input {
        r#type: "date",
        value: "{i_value}",
        class: "input-primary",
        oninput: move |event| on_input.call(event),
      }
    }
}

#[component]
pub fn SelectInput(
    i_value: String,
    on_input: EventHandler<FormEvent>,
    class: Option<String>,
    options: Vec<(String, String)>
) -> Element {
    rsx! {
      select {
        value: "{i_value}",
        class: "input-select",
        oninput: move |event| on_input.call(event),
        for (value, display) in options {
          option {
            value: value,
            "{display}"
          }
        }
      }
    }
}
