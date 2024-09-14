use dioxus::prelude::*;

use crate::LocationState;

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut location_state = consume_context::<Signal<LocationState>>();

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
        div {
            class: "menu-item {dashboard_active}",
            onclick: move |_| {
                if !location_state_value.is_dashboard() {
                    location_state.set(LocationState::Dashboard);
                }
            },
            "Dashboard"
        }
        div {
            class: "menu-item {logs_active}",
            onclick: move |_| {
                if !location_state_value.is_logs() {
                    location_state.set(LocationState::Logs);
                }
            },
            "Logs"
        }
        div {
            class: "menu-item {settings_active}",
            onclick: move |_| {
                if !location_state_value.is_settings() {
                    location_state.set(LocationState::Settings);
                }
            },
            "Settings"
        }
        div {
            class: "menu-item {ignore_single_events_active}",
            onclick: move |_| {
                if !location_state_value.is_ignore_single_events() {
                    location_state.set(LocationState::IgnoreSingleEvents);
                }
            },
            "Ignore Single Events"
        }
    }
}

//  Link { class: "menu-item {dashboard_active}", to: Route::Dashboard {}, "Dashboard" }
//  Link { class: "menu-item {logs_active}", to: Route::Home {}, "Logs" }
