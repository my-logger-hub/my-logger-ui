use dioxus::prelude::*;

use crate::main_state::{ActiveMenu, MainState};

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let mut dashboard_active = "";
    let mut logs_active = "";
    let mut settings_active = "";

    match main_state.read().menu.clone() {
        ActiveMenu::Dashboard => {
            dashboard_active = CLASS_NAME;
        }
        ActiveMenu::Logs => {
            logs_active = CLASS_NAME;
        }

        ActiveMenu::Settings(_) => {
            settings_active = CLASS_NAME;
        }
    }

    rsx! {
        div { id: "left-panel",
            div {
                h1 { style: "color:white; padding:5px; text-align:center", "Logs" }
            }
            br {}
            div { style: "padding: 5px" }
            div {
                class: "menu-item {dashboard_active}",
                onclick: move |_| {
                    main_state.write().set_menu(ActiveMenu::Dashboard);
                },
                "Dashboard"
            }
            div {
                class: "menu-item {logs_active}",
                onclick: move |_| {
                    main_state.write().set_menu(ActiveMenu::Logs);
                },
                "Logs"
            }
            div {
                class: "menu-item {settings_active}",
                onclick: move |_| {
                    main_state.write().set_menu(ActiveMenu::Settings(None));
                },
                "Settings"
            }
        }
    }
}

//  Link { class: "menu-item {dashboard_active}", to: Route::Dashboard {}, "Dashboard" }
//  Link { class: "menu-item {logs_active}", to: Route::Home {}, "Logs" }
