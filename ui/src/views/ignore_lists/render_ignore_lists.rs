use dioxus::prelude::*;

use crate::*;

use super::render_tabs::*;

#[component]
pub fn RenderIgnoreListsRoot() -> Element {
    let location_state = consume_context::<Signal<LocationState>>();

    let location = { location_state.read().copy_state() };

    let content = match location {
        LocationState::IgnoreList => {
            rsx! {
                RenderIgnoreList {}
            }
        }

        LocationState::OneTimeIgnore => {
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
