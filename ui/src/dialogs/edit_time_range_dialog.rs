use crate::dialogs::DialogTemplate;

use super::DialogState;

use dioxus::prelude::*;
use dioxus_utils::RenderState;
use rust_extensions::date_time::DateTimeAsMicroseconds;

use crate::{models::*, states::MainState};

/// How far the slider of the hours ago goes until the server says how long it keeps the hours
const DEFAULT_SLIDER_HOURS: i32 = 48;

#[component]
pub fn EditTimeRangeDialog(
    value: TimeRange,
    on_change: EventHandler<TimeRange>,
    time_zone: TimeZone,
) -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    // The hours are kept for `hours_to_gc`: there is nothing to read further than that
    let hours_kept = match consume_context::<Signal<MainState>>()
        .read()
        .server_settings
        .as_ref()
    {
        RenderState::Loaded(value) if value.hours_to_gc > 0 => value.hours_to_gc as i32,
        _ => DEFAULT_SLIDER_HOURS,
    };

    let mut time_range_state = use_signal(|| TimeRangeState::from(value));

    let time_range_value = time_range_state.read().get_current_value();

    let mut class_time_ago = "btn-outline-secondary";
    let mut class_range = "btn-outline-secondary";
    let mut class_exact_hour = "btn-outline-secondary";
    let to_render = match time_range_value {
        TimeRange::HoursAgo(v) => {
            class_time_ago = "btn-secondary";

            // A value typed in may be further than the hours are kept
            let slider_max = hours_kept.max(v);
            let hint = hours_ago_hint(v, time_zone);

            rsx! {
                div { style: "display: flex; align-items: flex-start; gap: 12px;",
                    div { style: "flex: 1; padding-top: 6px;",
                        // `max` goes before `value`: a value past the max of the moment is cut
                        input {
                            r#type: "range",
                            class: "form-range",
                            min: "0",
                            max: "{slider_max}",
                            step: "1",
                            value: "{v}",
                            oninput: move |e| {
                                let value = e.value().parse::<i32>().unwrap_or(0);
                                time_range_state.write().set_hours_ago(value);
                            },
                        }
                        div { style: "display: flex; justify-content: space-between; font-size: 11px; color: gray;",
                            span { "0" }
                            span { "{slider_max}" }
                        }
                    }
                    input {
                        r#type: "number",
                        class: "form-control",
                        // `.form-control` has no border of its own here, and next to the
                        // slider a bare number does not look like a field
                        style: "width: 90px; border: 1px solid lightgray;",
                        min: "0",
                        value: v.to_string(),
                        oninput: move |e| {
                            let value = e.value().parse::<i32>().unwrap_or(0);
                            time_range_state.write().set_hours_ago(value);
                        },
                    }
                }
                div { style: "margin-top: 8px; font-size: 12px; color: gray;", "{hint}" }
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
                                value: from.to_local_time(time_zone).to_string(),
                                oninput: move |e| {
                                    time_range_state.write().set_from_date(e.value(), time_zone);
                                },
                            }
                        }
                        td { " - " }
                        td {
                            input {
                                r#type: "datetime-local",
                                class: "form-control",
                                value: to.to_local_time(time_zone).to_string(),
                                oninput: move |e| {
                                    time_range_state.write().set_to_date(e.value(), time_zone);
                                },
                            }
                        }
                    }
                }
            }
        }
        TimeRange::ExactHour(key) => {
            class_exact_hour = "btn-secondary";

            let value = key
                .to_local_time(time_zone)
                .to_html_input_date_local_string();

            rsx! {
                input {
                    r#type: "datetime-local",
                    class: "form-control",
                    value: value.as_str(),
                    oninput: move |e| {
                        let value = e.value();
                        dioxus_utils::console_log(format!("Log: [{}]", value.as_str()).as_str());
                        time_range_state.write().set_exact_hour(e.value(), time_zone);
                    },
                }
            }
        }
    };
    rsx! {

        DialogTemplate {
            header: "Edit time range",
            dialog_state: dialog_state.clone(),
            modal_body: rsx! {
                div { class: "btn-group", style: "width: 100%;",
                    button {
                        class: "btn {class_time_ago}",
                        onclick: move |_| {
                            time_range_state.write().set_mode(SelectedRangeMode::HoursAgo);
                        },
                        "Hours ago"
                    }
                    button {
                        class: "btn {class_range}",
                        onclick: move |_| {
                            time_range_state.write().set_mode(SelectedRangeMode::Range);
                        },
                        "Date range"
                    }
                    button {
                        class: "btn {class_exact_hour}",
                        onclick: move |_| {
                            time_range_state.write().set_mode(SelectedRangeMode::ExactHour);
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
                        on_change.call(time_range_state.read().get_current_value());
                        dialog_state.set(DialogState::None);
                    },
                    "OK"
                }
            },
        }
    }
}

/// What the hours ago read: zero - the latest records, whichever hours they are in, the other
/// ones - that hour alone, shown as "2026-10-12 14:00 - 14:59, UTC+3"
fn hours_ago_hint(hours_ago: i32, time_zone: TimeZone) -> String {
    if hours_ago == 0 {
        return "The latest records".to_string();
    }

    let mut hour = DateTimeAsMicroseconds::now();
    hour.add_hours(-(hours_ago as i64));
    let hour: DateHourKey = hour.into();

    let from: DateTimeAsMicroseconds = hour.into();
    let mut to = from;
    to.add_minutes(59);

    let from = time_zone.to_local_time(from).to_rfc3339_utc();
    let to = time_zone.to_local_time(to).to_rfc3339_utc();

    format!(
        "{} {} - {}, {}",
        &from[..10],
        &from[11..16],
        &to[11..16],
        time_zone.to_string()
    )
}

#[derive(Debug, Clone)]
enum SelectedRangeMode {
    HoursAgo,
    ExactHour,
    Range,
}

#[derive(Debug, Clone)]
struct TimeRangeState {
    pub selected_range_mode: SelectedRangeMode,
    pub hours_ago: i32,
    pub from_date: HtmlInputDateTime,
    pub to_date: HtmlInputDateTime,
    pub exact_hour: DateHourKey,
}

impl TimeRangeState {
    pub fn from(src_value: TimeRange) -> Self {
        let now = rust_extensions::date_time::DateTimeAsMicroseconds::now();

        match src_value {
            TimeRange::HoursAgo(v) => {
                let mut before = now.clone();
                before.add_hours(-1);
                return Self {
                    selected_range_mode: SelectedRangeMode::HoursAgo,
                    hours_ago: v,
                    from_date: before.into(),
                    to_date: now.into(),
                    exact_hour: now.into(),
                };
            }
            TimeRange::Range(from, to) => {
                return Self {
                    selected_range_mode: SelectedRangeMode::Range,
                    hours_ago: 0,
                    from_date: from,
                    to_date: to,
                    exact_hour: now.into(),
                };
            }
            TimeRange::ExactHour(v) => {
                let mut before = now.clone();
                before.add_hours(-1);
                return Self {
                    selected_range_mode: SelectedRangeMode::ExactHour,
                    hours_ago: 0,
                    from_date: before.into(),
                    to_date: now.into(),
                    exact_hour: v,
                };
            }
        }
    }

    pub fn set_hours_ago(&mut self, value: i32) {
        if value < 0 {
            self.hours_ago = -value as i32;
        } else {
            self.hours_ago = value as i32;
        }
    }

    pub fn set_from_date(&mut self, value: String, time_zone: TimeZone) {
        if let Some(value) = HtmlInputDateTime::try_from_str(&value) {
            self.from_date = value.to_utc_time(time_zone);
        }
    }

    pub fn set_to_date(&mut self, value: String, time_zone: TimeZone) {
        if let Some(value) = HtmlInputDateTime::try_from_str(&value) {
            self.to_date = value.to_utc_time(time_zone);
        }
    }

    pub fn set_exact_hour(&mut self, value: String, time_zone: TimeZone) {
        if let Some(key) = DateHourKey::try_from_str(&value) {
            self.exact_hour = key.to_utc_time(time_zone);
        }
    }

    pub fn set_mode(&mut self, selected_range_mode: SelectedRangeMode) {
        self.selected_range_mode = selected_range_mode;
    }

    pub fn get_current_value(&self) -> TimeRange {
        match self.selected_range_mode {
            SelectedRangeMode::HoursAgo => TimeRange::HoursAgo(self.hours_ago),
            SelectedRangeMode::Range => {
                TimeRange::Range(self.from_date.clone(), self.to_date.clone())
            }
            SelectedRangeMode::ExactHour => TimeRange::ExactHour(self.exact_hour),
        }
    }
}
