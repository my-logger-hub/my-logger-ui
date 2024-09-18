use dioxus::prelude::*;

use crate::*;

use super::render_tabs::*;

#[component]
pub fn RenderSettings() -> Element {
    let location_state = consume_context::<Signal<LocationState>>();

    let location = { location_state.read().copy_state() };

    let content = match location {
        LocationState::SettingsIgnoreList => {
            rsx! {
                RenderIgnoreList {}
            }
        }

        LocationState::SettingsOneTimeIgnore => {
            rsx! {
                RenderOneTimeIgnoreList {}
            }
        }

        _ => {
            rsx! {
                div {}
            }
        }
    };

    rsx! {
        RenderTabs {}

        {content}
    }
}
