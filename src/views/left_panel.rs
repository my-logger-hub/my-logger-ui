use dioxus::prelude::*;

use crate::{LocationState, MainState};

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut dashboard_active = "";
    let mut logs_active = "";
    let mut settings_active = "";

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

        LocationState::SettingsIgnoreList => {
            settings_active = CLASS_NAME;
        }

        LocationState::SettingsOneTimeIgnore => {
            settings_active = CLASS_NAME;
        }
    }

    let time_zone = {
        let main_state = consume_context::<Signal<MainState>>();
        let read_access = main_state.read();
        read_access.time_zone
    };

    let time_zone = if time_zone < 0 {
        format!("TimeZone: UTC+{}", -time_zone as f64 / 60.0)
    } else {
        format!("TimeZone: UTC{}", -time_zone as f64 / 60.0)
    };

    rsx! {

        div {
            h1 { style: "color:white; padding:5px; text-align:center", "Logs" }
        }

        div { style: "    color: white;text-align: center;", "{time_zone}" }

        div { style: "padding: 5px" }

        Link { class: "menu-item {dashboard_active}", to: "/", "Dashboard" }

        Link { class: "menu-item {logs_active}", to: "/logs", "Logs" }

        Link { class: "menu-item {settings_active}", to: "/settings", "Settings" }
    }
}

//  Link { class: "menu-item {dashboard_active}", to: Route::Dashboard {}, "Dashboard" }
//  Link { class: "menu-item {logs_active}", to: Route::Home {}, "Logs" }
