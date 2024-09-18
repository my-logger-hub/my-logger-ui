use dioxus::prelude::*;

use crate::storage_settings::log_level::SelectedLevel;

#[component]
pub fn SelectLogLevel(value: SelectedLevel, on_change: EventHandler<SelectedLevel>) -> Element {
    let items = SelectedLevel::LEVELS.map(|(s_key, s_value)| {
        if s_key == value.as_str() {
            rsx! {
                option { selected: true, value: s_key, {s_value} }
            }
        } else {
            rsx! {
                option { value: s_key, {s_value} }
            }
        }
    });
    rsx! {
        select {
            class: "form-select form-select-sm",
            oninput: move |e| {
                let level = SelectedLevel::from_str(e.value().as_str());
                on_change.call(level);
            },
            value: value.as_str(),
            {items.into_iter()}
        }
    }
}
