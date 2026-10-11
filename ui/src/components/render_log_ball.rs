use dioxus::prelude::*;

use crate::models::*;

pub fn render_log_ball(level: LogApiLevel) -> Element {
    let style = match level {
        LogApiLevel::Info => "green-ball",
        LogApiLevel::Warning => "orange-ball",
        LogApiLevel::Error => "red-ball",
        LogApiLevel::FatalError => "dark-red-ball",
        LogApiLevel::Debug => "gray-ball",
    };

    rsx! {
        div { class: "{style}" }
    }
}
