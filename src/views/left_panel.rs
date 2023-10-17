use dioxus::prelude::*;

use crate::states::*;

const CLASS_NAME: &str = "menu-item-active";
pub fn left_panel(cx: Scope) -> Element {
    let left_panel_state = use_shared_state::<LeftMenuState>(cx).unwrap();

    let mut home_active = "";
    let mut settings_active = "";

    match *left_panel_state.read() {
        LeftMenuState::Home => {
            home_active = CLASS_NAME;
        }
        LeftMenuState::Settings => {
            settings_active = CLASS_NAME;
        }
    }

    render! {
        div { id: "left-panel",
            div { h1 { style: "color:white; padding:5px; text-align:center", "Logs" } }
            br {}
            div { style: "padding: 5px",
                div {
                    class: "menu-item {home_active}",
                    onclick: move |_| {
                        *left_panel_state.write() = LeftMenuState::Home;
                    },
                    "Home"
                }
                div {
                    class: "menu-item {settings_active}",
                    onclick: move |_| {
                        *left_panel_state.write() = LeftMenuState::Settings;
                    },
                    "Settings"
                }
            }
        }
    }
}
