use std::rc::Rc;

use dioxus::prelude::*;

use crate::{models::ServerInfoModel, DataState, LocationState, MainState};

const CLASS_NAME: &str = "menu-item-active";
#[component]
pub fn LeftPanel() -> Element {
    let mut dashboard_active = "";
    let mut logs_active = "";
    let mut settings_active = "";
    let mut ignore_lists_active = "";

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

        LocationState::IgnoreList => {
            ignore_lists_active = CLASS_NAME;
        }

        LocationState::OneTimeIgnore => {
            ignore_lists_active = CLASS_NAME;
        }

        LocationState::Settings => {
            settings_active = CLASS_NAME;
        }
    }
    let mut main_state = consume_context::<Signal<MainState>>();
    let (time_zone, server_settings, env) = {
        let read_access = main_state.read();
        (
            read_access.get_selected_timezone(),
            read_access.server_settings.clone(),
            read_access.get_selected_env(),
        )
    };

    let server_settings = match server_settings {
        DataState::None => {
            main_state.write().server_settings = DataState::Loading;

            spawn(async move {
                let result = get_server_info(env.to_string()).await;

                match result {
                    Ok(result) => {
                        main_state.write().server_settings = DataState::Loaded(Rc::new(result));
                    }
                    Err(err) => {
                        main_state.write().server_settings = DataState::Error(err.to_string());
                    }
                }
            });

            None
        }
        DataState::Loading => None,

        DataState::Loaded(value) => Some(value),
        DataState::Error(err) => {
            return rsx! { "Error loading from server: {err}" };
        }
    };

    let time_zone = format!("TimeZone: {}", time_zone.to_string());

    let client_version = env!("CARGO_PKG_VERSION");
    let client_version = rsx! {

        div { "Client ver: {client_version}" }
    };

    let server_info = match server_settings {
        Some(server_settings) => {
            if server_settings.version.is_empty() {
                client_version
            } else {
                let gc_timeout = format_hours_to_gc(server_settings.hours_to_gc);

                rsx! {
                    {client_version},
                    div { "Server ver: {server_settings.version.as_str()}" }
                    div { "GC: {gc_timeout.as_str()}" }
                }
            }
        }

        None => client_version,
    };

    rsx! {

        div {
            h1 { style: "color:white; padding:5px; text-align:center", "Logs" }
        }

        div { style: "    color: white;text-align: center;", "{time_zone}" }

        div { style: "padding: 5px" }

        Link { class: "menu-item {dashboard_active}", to: "/", "Dashboard" }

        Link {
            class: "menu-item {logs_active}",
            onclick: move |_| {
                crate::storage_settings::time_range::clear();
                crate::storage_settings::search_line::clear();
                crate::storage_settings::log_level::clear();
            },
            to: "/logs",
            "Logs"
        }

        Link { class: "menu-item {ignore_lists_active}", to: "/ignoreLists", "Ignore Lists" }

        Link { class: "menu-item {settings_active}", to: "/settings", "Settings" }

        div { class: "server-info", {server_info} }
    }
}

#[server]
async fn get_server_info(env: String) -> Result<ServerInfoModel, ServerFnError> {
    match crate::APP_CTX
        .get_client(env.as_str())
        .await
        .get_server_info(())
        .await
    {
        Ok(result) => Ok(ServerInfoModel {
            version: result.version,
            hours_to_gc: result.hours_to_gc,
        }),
        Err(_) => Ok(ServerInfoModel {
            version: String::new(),
            hours_to_gc: 0,
        }),
    }
}

fn format_hours_to_gc(value: u32) -> String {
    if value < 24 {
        format!("{}h", value)
    } else {
        let days = value / 24;
        let hours = value % 24;
        format!("{}d {}h", days, hours)
    }
}

#[cfg(test)]
mod tests {
    use super::format_hours_to_gc;

    #[test]
    fn test() {
        let result = format_hours_to_gc(23);

        assert_eq!(result, "23h");

        let result = format_hours_to_gc(24);

        assert_eq!(result, "1d 0h");

        let result = format_hours_to_gc(25);

        assert_eq!(result, "1d 1h");
    }
}
