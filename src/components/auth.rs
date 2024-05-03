#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::{
    client::auth::User,
    components::{
        button::Button,
        input::{PasswordInput, TextInput},
    },
};

#[component]
pub fn Auth(on_success: EventHandler<User>) -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());

    let login = move |_| {
        spawn(async move {
            client! {
              let res = signin_with_password(Credentials { email: email.to_string(), password: password.to_string() }).await;

              if let Ok(session) = res {
                email.set("".into());
                password.set("".into());
                on_success.call(session.user);
              } else {
                alert("failed to login user");
              }
            }
        });
    };

    rsx! {
      label {
        "Email"
        TextInput {
            i_value: email,
            on_input: move |event: FormEvent| {
              email.set(event.value());
            },
        },
      }
      label {
        "Password"
        PasswordInput {
            i_value: password,
            on_input: move |event: FormEvent| {
              password.set(event.value());
            },
        },
      }
      Button {
        text: "Login",
        on_click: login
      }
    }
}
