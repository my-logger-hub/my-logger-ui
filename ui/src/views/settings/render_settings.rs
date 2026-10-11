use dioxus::prelude::*;

use crate::{storage_settings::selected_time_zone::SelectedTimeZone, MainState};

#[component]
pub fn RenderSettings() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();
    let (time_zone, selected_timezone) = {
        let read_access = main_state.read();
        (
            read_access.get_local_timezone(),
            read_access.selected_time_zone,
        )
    };

    let time_zone_as_string = time_zone.to_string();

    let options = match selected_timezone {
        SelectedTimeZone::UtcZero => {
            rsx! {
                option { selected: true, value: "0", "UTC+0" }
                option { value: "l", "Local Time: {time_zone_as_string}" }
            }
        }
        SelectedTimeZone::LocalTime => {
            rsx! {
                option { value: "0", "UTC+0" }
                option { selected: true, value: "l", "Local Time: {time_zone_as_string}" }
            }
        }
    };

    rsx! {
        h1 { "Settings" }

        div { style: "padding: 10px; width:50%;",
            div { class: "edit-wrapper",
                label { "Time Zone" }
                select {
                    class: "form-select",
                    oninput: move |c| {
                        let selected_time_zone = SelectedTimeZone::from_str(c.value().as_str());
                        crate::storage_settings::selected_time_zone::set(selected_time_zone);
                        main_state.write().selected_time_zone = selected_time_zone;
                    },
                    {options}
                }
            }
        }
    }
}
