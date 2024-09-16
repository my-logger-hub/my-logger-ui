use dioxus::prelude::*;

use crate::LocationState;

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut dashboard_active = "";
    let mut logs_active = "";
    let mut settings_active = "";
    let mut ignore_single_events_active = "";

    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    match location_state_value {
        LocationState::Dashboard => {
            dashboard_active = CLASS_NAME;
        }
        LocationState::Logs => {
            logs_active = CLASS_NAME;
        }

        LocationState::Settings => {
            settings_active = CLASS_NAME;
        }

        LocationState::IgnoreSingleEvents => {
            ignore_single_events_active = CLASS_NAME;
        }
    }

    rsx! {

        div {
            h1 { style: "color:white; padding:5px; text-align:center", "Logs" }
        }
        br {}
        div { style: "padding: 5px" }

        Link { class: "menu-item {dashboard_active}", to: "/", "Dashboard" }

        Link { class: "menu-item {logs_active}", to: "/logs", "Logs" }

        Link { class: "menu-item {settings_active}", to: "/settings", "Settings" }

        Link {
            class: "menu-item {ignore_single_events_active}",
            to: "/ignore_single_events",
            "Ignore Single Events"
        }
    }
}

//  Link { class: "menu-item {dashboard_active}", to: Route::Dashboard {}, "Dashboard" }
//  Link { class: "menu-item {logs_active}", to: Route::Home {}, "Logs" }
