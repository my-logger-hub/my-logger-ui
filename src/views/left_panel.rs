use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::icon;
use crate::views::EnvsSelector;
use crate::{LocationState, MainState};

use dioxus_utils::*;

const ACTIVE: &str = "is-active";

#[component]
pub fn LeftPanel() -> Element {
    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    let dashboard_active = matches!(location_state_value, LocationState::Dashboard);
    let logs_active = matches!(location_state_value, LocationState::Logs);
    let ignore_active = matches!(
        location_state_value,
        LocationState::IgnoreList | LocationState::OneTimeIgnore
    );
    let settings_active = matches!(location_state_value, LocationState::Settings);

    let class_of = |active: bool| {
        if active {
            format!("ml-side__navitem {ACTIVE}")
        } else {
            "ml-side__navitem".to_string()
        }
    };

    let mut main_state = consume_context::<Signal<MainState>>();
    let main_state_ra = main_state.read();

    let server_settings = match main_state_ra.server_settings.as_ref() {
        RenderState::None => {
            let selected_env = main_state_ra.get_selected_env();
            spawn(async move {
                main_state.write().server_settings.set_loading();
                let result =
                    crate::api::server_info::get_server_info(selected_env.to_string()).await;

                match result {
                    Ok(result) => {
                        main_state.write().server_settings.set_value(Rc::new(result));
                    }
                    Err(err) => {
                        main_state.write().server_settings.set_error(err.to_string());
                    }
                }
            });
            None
        }
        RenderState::Loading => None,
        RenderState::Loaded(value) => Some(value.clone()),
        RenderState::Error(_) => None,
    };

    let time_zone = main_state_ra.get_selected_timezone().to_string();

    let client_version = env!("CARGO_PKG_VERSION");

    let (server_version, gc_value, connected) = match server_settings.as_ref() {
        Some(server_settings) if !server_settings.version.is_empty() => (
            server_settings.version.clone(),
            format_hours_to_gc(server_settings.hours_to_gc),
            true,
        ),
        _ => ("…".to_string(), "…".to_string(), false),
    };

    let status = if connected {
        rsx! {
            span { class: "ml-status",
                span { class: "dot" }
                "connected"
            }
        }
    } else {
        rsx! {
            span { class: "ml-status", "…" }
        }
    };

    rsx! {
        aside { class: "ml-side", role: "navigation", aria_label: "primary",
            div { class: "ml-side__brand",
                div { class: "ml-side__logo", "L" }
                div { class: "ml-side__title", "Logs" }
            }

            div { class: "ml-side__env",
                div { class: "ml-side__label", "Environment" }
                EnvsSelector {}
            }

            div { class: "ml-side__tz",
                span { "TimeZone" }
                span { style: "color: var(--ml-side-fg);", "{time_zone}" }
            }

            nav { class: "ml-side__nav",
                Link { class: class_of(dashboard_active), to: "/",
                    span { class: "ico", {icon("dashboard", 14)} }
                    span { "Dashboard" }
                }
                Link {
                    class: class_of(logs_active),
                    onclick: move |_| {
                        crate::storage_settings::time_range::clear();
                        crate::storage_settings::search_line::clear();
                        crate::storage_settings::log_level::clear();
                    },
                    to: "/logs",
                    span { class: "ico", {icon("logs", 14)} }
                    span { "Logs" }
                }
                Link { class: class_of(ignore_active), to: "/ignoreLists",
                    span { class: "ico", {icon("ignore", 14)} }
                    span { "Ignore Lists" }
                }
                Link { class: class_of(settings_active), to: "/settings",
                    span { class: "ico", {icon("settings", 14)} }
                    span { "Settings" }
                }
            }

            div { class: "ml-side__footer",
                div { class: "row",
                    dt { "Client ver" }
                    dd { "{client_version}" }
                }
                div { class: "row",
                    dt { "Server ver" }
                    dd { "{server_version}" }
                }
                div { class: "row",
                    dt { "GC" }
                    dd { "{gc_value}" }
                }
                div { class: "row",
                    dt { "Status" }
                    dd { {status} }
                }
            }
        }
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
