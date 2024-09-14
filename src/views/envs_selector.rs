use dioxus::prelude::*;

use crate::states::*;

#[component]
pub fn EnvsSelector() -> Element {
    let main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let envs_options = if let Some(envs) = main_state_read_access.envs.as_ref() {
        envs.clone().into_iter().map(|env| {
            if env.as_str() == main_state_read_access.selected_env.as_str() {
                rsx! {
                    option { selected: true, {env.as_str() } }
                }
            } else {
                rsx! {
                    option { {env.as_str() } }
                }
            }
        })
    } else {
        return rsx! {
            option {}
        };
    };

    rsx! {
        select {
            class: "form-select",
            style: "background-color: white;",

            value: main_state_read_access.selected_env.as_str(),

            oninput: |ctx| {
                let value = ctx.value();
                consume_context::<Signal<MainState>>().write().set_active_env(value.as_str());
            },
            {envs_options}
        }
    }
}
