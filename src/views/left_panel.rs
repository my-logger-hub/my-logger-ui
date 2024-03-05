use dioxus::prelude::*;

use crate::main_state::MainState;

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let mut dashboard_active = "";
    let mut logs_active = "";
    let mut settings_active = "";

    match main_state.read().clone() {
        MainState::Dashboard => {
            dashboard_active = CLASS_NAME;
        }
        MainState::Logs => {
            logs_active = CLASS_NAME;
        }

        MainState::Settings => {
            settings_active = CLASS_NAME;
        }
    }

    rsx! {
        div { id: "left-panel",
            div { h1 { style: "color:white; padding:5px; text-align:center", "Logs" } }
            br {}
            div { style: "padding: 5px" }
            div {
                class: "menu-item {dashboard_active}",
                onclick: move |_| {
                    main_state.set(MainState::Dashboard);
                },
                "Dashboard"
            }
            div {
                class: "menu-item {logs_active}",
                onclick: move |_| {
                    main_state.set(MainState::Logs);
                },
                "Logs"
            }
            div {
                class: "menu-item {settings_active}",
                onclick: move |_| {
                    main_state.set(MainState::Settings);
                },
                "Settings"
            }
        }
    }
}

//  Link { class: "menu-item {dashboard_active}", to: Route::Dashboard {}, "Dashboard" }
//  Link { class: "menu-item {logs_active}", to: Route::Home {}, "Logs" }
