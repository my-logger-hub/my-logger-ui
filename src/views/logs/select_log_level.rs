use dioxus::prelude::*;

use crate::storage_settings::log_level::SelectedLevel;

#[component]
pub fn SelectLogLevel() -> Element {
    let mut log_level_filter = use_signal(|| crate::storage_settings::log_level::get());

    let active = log_level_filter.read().clone();

    let items = SelectedLevel::LEVELS.map(|(key, value)| {
        if key == active.as_str() {
            rsx! {
                option { selected: true, value: key, {value} }
            }
        } else {
            rsx! {
                option { value: key, {value} }
            }
        }
    });
    rsx! {
        select {
            class: "form-select form-select-sm",
            oninput: move |e| {
                let level = SelectedLevel::from_str(e.value().as_str());
                crate::storage_settings::log_level::set(level);
                log_level_filter.set(level);
            },
            value: log_level_filter.read().as_str(),
            {items.into_iter()}
        }
    }
}
