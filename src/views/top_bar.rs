use dioxus::prelude::*;

use crate::components::icon;
use crate::storage_settings::theme::{self, Theme};
use crate::{LocationState, MainState};

#[component]
pub fn TopBar() -> Element {
    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    let title = match location_state_value {
        LocationState::Dashboard => "Dashboard",
        LocationState::Logs => "Logs",
        LocationState::IgnoreList | LocationState::OneTimeIgnore => "Ignore Lists",
        LocationState::Settings => "Settings",
    };

    let main_state = consume_context::<Signal<MainState>>();
    let main_state_ra = main_state.read();
    let env = main_state_ra.get_selected_env();
    let tz = main_state_ra.get_selected_timezone().to_string();
    drop(main_state_ra);

    let mut current_theme = use_signal(theme::get);
    let theme_value = *current_theme.read();
    let (theme_icon, theme_label) = match theme_value {
        Theme::Light => ("moon", "Switch to dark theme"),
        Theme::Dark => ("sun", "Switch to light theme"),
    };

    rsx! {
        div { class: "ml-topbar",
            div { class: "ml-topbar__title",
                span { "{title}" }
                span { class: "ml-topbar__crumb", " / {env}" }
            }
            div { class: "ml-topbar__right",
                span { class: "ml-pill",
                    "env "
                    strong { "{env}" }
                }
                span { class: "ml-pill",
                    "tz "
                    strong { "{tz}" }
                }
                button {
                    class: "ml-btn ml-btn--icon",
                    aria_label: "{theme_label}",
                    title: "{theme_label}",
                    onclick: move |_| {
                        let next = current_theme.read().toggled();
                        theme::set(next);
                        current_theme.set(next);
                    },
                    {icon(theme_icon, 14)}
                }
            }
        }
    }
}
