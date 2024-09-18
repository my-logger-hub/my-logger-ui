use dioxus::prelude::*;

use crate::{LocationState, Route, IGNORE_SINGLE_TIME_SUB_PATH};
#[component]
pub fn RenderTabs() -> Element {
    let mut location_state = consume_context::<Signal<LocationState>>();
    let location = { location_state.read().copy_state() };
    let mut ignore_list_active = "";
    let mut one_time_ignore_active = "";

    match location {
        LocationState::SettingsOneTimeIgnore => {
            one_time_ignore_active = "active";
        }

        _ => {
            ignore_list_active = "active";
        }
    }

    rsx! {

        ul { class: "nav nav-tabs",
            li { class: "nav-item",
                Link {
                    to: Route::Settings { data: vec![] },
                    class: "nav-link {ignore_list_active}",
                    onclick: move |_| {
                        location_state.set(LocationState::SettingsIgnoreList);
                    },
                    "Ignore list"
                }
            }
            li { class: "nav-item",
                Link {
                    to: Route::Settings {
                        data: vec![IGNORE_SINGLE_TIME_SUB_PATH.to_string()],
                    },
                    class: "nav-link {one_time_ignore_active}",
                    onclick: move |_| {
                        location_state.set(LocationState::SettingsOneTimeIgnore);
                    },
                    "One time ignore"
                }
            }
        }
    }
}
