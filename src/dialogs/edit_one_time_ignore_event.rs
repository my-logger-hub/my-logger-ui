use std::rc::Rc;

use dioxus::prelude::*;

use crate::{components::*, dialogs::DialogTemplate, LogApiLevel, OneTimeIgnoreHttpModel};

use super::DialogState;

pub fn edit_one_time_ignore_event(
    mut dialog_state: Signal<DialogState>,
    value: Rc<OneTimeIgnoreHttpModel>,
    on_ok: EventHandler<OneTimeIgnoreHttpModel>,
) -> Element {
    let mut state = use_signal(|| EditOneTimeIgnoreEventState::new(&value));

    let state_read_access = state.read();

    let btn_disabled = !state_read_access.validate();

    let select_items = state_read_access.levels.iter().map(|itm| {
        let log_api_level = itm.0.clone();
        rsx! {
            tr {
                td {
                    InputBool {
                        value: itm.1,
                        on_change: move |v| {
                            for itm in state.write().levels.iter_mut() {
                                if itm.0 == log_api_level {
                                    itm.1 = v;
                                }
                            }
                        }
                    }
                }
                td { {itm.0.as_str()} }
            }
        }
    });

    let key_value_to_render = state_read_access.ctx.iter().map(|(key, value)| {
        rsx! {
            div { "{key}={value}" }
        }
    });

    rsx! {
        DialogTemplate {
            header: "Edit one time ignore event",
            dialog_state: dialog_state.clone(),
            modal_body: rsx! {
                table {
                    tr {
                        td { {select_items} }
                        td { style: "vertical-align: top;padding-left: 10px; width:100%",
                            InputString {
                                caption: "Message matching",
                                value: state_read_access.message_match.as_str(),
                                on_input: move |v| {
                                    state.write().message_match = v;
                                },
                                on_enter_pressed: None
                            }
                            InputI64 {
                                caption: "Minutes to wait",
                                value: state_read_access.minutes_to_wait,
                                on_input: move |v| {
                                    if let InputValue::Value(v) = v {
                                        state.write().minutes_to_wait = v;
                                    }
                                }
                            }
                            InputI64 {
                                caption: "Skip amount",
                                value: state_read_access.skip_amount,
                                on_input: move |v| {
                                    if let InputValue::Value(v) = v {
                                        state.write().skip_amount = v;
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "edit-wrapper", {key_value_to_render} }
                tr {
                    td {
                        InputString {
                            caption: " Key",
                            value: state_read_access.current_key.as_str(),
                            on_input: move |v| {
                                state.write().current_key = v;
                            }
                        }
                    }
                    td {
                        InputString {
                            caption: " Value",
                            value: state_read_access.current_value.as_str(),
                            on_input: move |v| {
                                state.write().current_value = v;
                            }
                        }
                    }
                    td {
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| {
                                let mut state_write_access = state.write();
                                let key = state_write_access.current_key.to_string();
                                let value = state_write_access.current_value.to_string();
                                state_write_access.ctx.push((key, value));
                                state_write_access.current_key = String::new();
                                state_write_access.current_value = String::new();
                            },
                            "Add"
                        }
                    }
                }
            },
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let model = state.read().to_http_model();
                        dialog_state.set(DialogState::None);
                        on_ok.call(model);
                    },
                    disabled: btn_disabled,
                    "OK"
                }
            }
        }
    }
}

pub struct EditOneTimeIgnoreEventState {
    pub id: String,
    pub levels: Vec<(LogApiLevel, bool)>,
    pub message_match: String,
    pub skip_amount: i64,
    pub minutes_to_wait: i64,

    pub ctx: Vec<(String, String)>,

    pub current_key: String,
    pub current_value: String,
}

impl EditOneTimeIgnoreEventState {
    pub fn new(src: &OneTimeIgnoreHttpModel) -> Self {
        let mut levels = Vec::new();
        for level in LogApiLevel::ALL_LEVELS.iter() {
            let checked = src.levels.contains(level);
            levels.push((level.clone(), checked));
        }

        Self {
            id: src.id.clone(),
            levels,
            current_key: String::new(),
            current_value: String::new(),
            message_match: src.message_match.clone(),
            skip_amount: src.skip_amount as i64,
            minutes_to_wait: src.minutes_to_wait as i64,
            ctx: src
                .ctx_match
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect(),
        }
    }

    pub fn to_http_model(&self) -> OneTimeIgnoreHttpModel {
        OneTimeIgnoreHttpModel {
            id: self.id.clone(),
            levels: self
                .levels
                .iter()
                .filter(|(_, v)| *v)
                .map(|(k, _)| k.clone())
                .collect(),
            message_match: self.message_match.clone(),
            skip_amount: self.skip_amount as u64,
            minutes_to_wait: self.minutes_to_wait as u64,
            ctx_match: self
                .ctx
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }

    pub fn validate(&self) -> bool {
        if self.levels.iter().all(|itm| itm.1 == false) {
            return false;
        }

        if self.message_match.is_empty() {
            return false;
        }

        if self.skip_amount <= 0 {
            return false;
        }

        if self.minutes_to_wait <= 0 {
            return false;
        }

        true
    }
}
