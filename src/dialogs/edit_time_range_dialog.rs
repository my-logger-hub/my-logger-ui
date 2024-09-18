use crate::dialogs::DialogTemplate;

use super::DialogState;
use crate::date_key::DateHourKey;
use dioxus::prelude::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

pub fn edit_time_range_dialog(
    mut dialog_state: Signal<DialogState>,
    value: TimeRange,
    on_change: EventHandler<TimeRange>,
    time_zone: i64,
) -> Element {
    let mut time_range_state = use_signal(|| value);

    let mut temp_values = use_signal(|| {
        let mut now = dioxus_utils::js::now_date_time();
        now.add_minutes(-time_zone);
        let mut before = now.clone();
        before.add_hours(-1);

        TempValues {
            hours_ago: 0,
            from_date: before.to_rfc3339()[..16].to_string(),
            to_date: now.to_rfc3339()[..16].to_string(),
            exact_hour: now.into(),
        }
    });

    let time_range_value = time_range_state.read().clone();

    let mut class_time_ago = "btn-outline-secondary";
    let mut class_range = "btn-outline-secondary";
    let mut class_exact_hour = "btn-outline-secondary";
    let to_render = match time_range_value {
        TimeRange::HoursAgo(v) => {
            class_time_ago = "btn-secondary";

            rsx! {

                input {
                    r#type: "number",
                    class: "form-control",
                    value: v.to_string(),
                    oninput: move |e| {
                        let value = e.value().parse::<i32>().unwrap_or(0);
                        time_range_state.set(TimeRange::HoursAgo(value));
                        temp_values.write().hours_ago = value;
                    }
                }
            }
        }
        TimeRange::Range(from, to) => {
            class_range = "btn-secondary";

            rsx! {

                table { style: "width:100%",
                    tr {
                        td {
                            input {
                                r#type: "datetime-local",
                                class: "form-control",
                                value: from,
                                oninput: move |e| {
                                    let value = e.value();
                                    {
                                        temp_values.write().from_date = value.to_string();
                                    }
                                    *time_range_state.write().unwrap_as_range_mut().0 = value;
                                }
                            }
                        }
                        td { " - " }
                        td {
                            input {
                                r#type: "datetime-local",
                                class: "form-control",
                                value: to,
                                oninput: move |e| {
                                    let value = e.value();
                                    {
                                        temp_values.write().to_date = value.to_string();
                                    }
                                    *time_range_state.write().unwrap_as_range_mut().1 = value;
                                }
                            }
                        }
                    }
                }
            }
        }
        TimeRange::ExactHour(key) => {
            class_exact_hour = "btn-secondary";

            let value = key.to_html_input_date_local_string();

            dioxus_utils::js::console_log(value.as_str());

            rsx! {
                input {
                    r#type: "datetime-local",
                    class: "form-control",
                    value: value.as_str(),
                    oninput: move |e| {
                        let value = e.value();
                        let key = DateHourKey::try_from_str(&value).unwrap();
                        dioxus_utils::js::console_log(key.to_string().as_str());
                        time_range_state.set(TimeRange::ExactHour(key));
                        temp_values.write().exact_hour = key;
                    }
                }
            }
        }
    };
    rsx! {

        DialogTemplate {
            dialog_state: dialog_state.clone(),
            modal_body: rsx! {
                div { class: "btn-group", style: "width: 100%;",
                    button {
                        class: "btn {class_time_ago}",
                        onclick: move |_| {
                            let hour = temp_values.read().hours_ago;
                            time_range_state.set(TimeRange::HoursAgo(hour));
                        },
                        "Hours ago"
                    }
                    button {
                        class: "btn {class_range}",
                        onclick: move |_| {
                            let (before, after) = {
                                let temp_values = temp_values.read();
                                (temp_values.from_date.to_string(), temp_values.to_date.to_string())
                            };
                            time_range_state.set(TimeRange::Range(before, after));
                        },
                        "Date range"
                    }
                    button {
                        class: "btn {class_exact_hour}",
                        onclick: move |_| {
                            let value = temp_values.read().exact_hour;
                            time_range_state.set(TimeRange::ExactHour(value));
                        },
                        "Exact hour"
                    }
                }
                div { style: "margin-top:20px", {to_render} }
            },
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        on_change.call(time_range_state.read().clone());
                        dialog_state.set(DialogState::None);
                    },
                    "OK"
                }
            }
        }
    }
}

pub struct TempValues {
    pub hours_ago: i32,
    pub from_date: String,
    pub to_date: String,
    pub exact_hour: DateHourKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeRange {
    HoursAgo(i32),
    Range(String, String),
    ExactHour(DateHourKey),
}

impl TimeRange {
    pub fn default() -> Self {
        TimeRange::HoursAgo(0)
    }
    pub fn to_string(&self) -> String {
        match self {
            TimeRange::HoursAgo(v) => v.to_string(),
            TimeRange::Range(start, end) => format!("{} - {}", start, end),
            TimeRange::ExactHour(k) => k.to_string(),
        }
    }

    pub fn unwrap_as_range_mut(&mut self) -> (&mut String, &mut String) {
        match self {
            TimeRange::Range(start, end) => (start, end),
            _ => panic!("Not a range"),
        }
    }

    pub fn get_date_from_date_to(&self, time_zone: i64) -> (i64, i64) {
        match self {
            Self::HoursAgo(value) => (*value as i64, 0),
            Self::Range(from, to) => {
                let mut from = DateTimeAsMicroseconds::from_str(from).unwrap();
                from.add_minutes(time_zone);
                let mut to = DateTimeAsMicroseconds::from_str(to).unwrap();
                to.add_minutes(time_zone);

                (from.unix_microseconds, to.unix_microseconds)
            }
            Self::ExactHour(value) => {
                let mut dt: DateTimeAsMicroseconds = value.into();

                dt.add_minutes(time_zone);

                let result: DateHourKey = dt.into();
                (result.get_value(), 0)
            }
        }
    }

    pub fn from_str(value: &str) -> Self {
        //2024-09-17T22:19 - 2024-09-17T23:19
        if value.len() > 30 {
            let from = value[..16].to_string();
            let to = value[19..].to_string();

            let from_dt = DateTimeAsMicroseconds::from_str(from.as_str());
            let to_dt = DateTimeAsMicroseconds::from_str(to.as_str());

            if from_dt.is_some() && to_dt.is_some() {
                return TimeRange::Range(from, to);
            }
        }

        let number = value.parse::<i32>();

        if let Ok(number) = number {
            return TimeRange::HoursAgo(number);
        }

        let date_hour_key = DateHourKey::try_from_str(value);

        if let Some(date_hour_key) = date_hour_key {
            return TimeRange::ExactHour(date_hour_key);
        }

        return TimeRange::default();
    }
}
