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

    let stl = match value {
        SelectedLevel::All => "",
        SelectedLevel::FatalError => "background: black; color:white;",
        SelectedLevel::Error => "background: red; color:white;",
        SelectedLevel::Warning => "background: orange;",
        SelectedLevel::Info => "color: darkgreen;",
        SelectedLevel::Debug => "background: lightgray;color: black;",
    };

    rsx! {
        select {
            class: "form-select form-select-sm",
            style: stl,
            oninput: move |e| {
                let level = SelectedLevel::from_str(e.value().as_str());
                on_change.call(level);
            },
            value: value.as_str(),
            {items.into_iter()}
        }
    }
}
