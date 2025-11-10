use crate::dialogs::DialogTemplate;
use dioxus::prelude::*;

use super::DialogState;

use crate::models::*;

#[component]
pub fn RenderAddIgnoreEvent(on_ok: EventHandler<IgnoreEventApiModel>) -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let mut state = use_signal(|| EventConfirmationState::new());

    rsx! {
        DialogTemplate {
            header: "Add ignore event",
            dialog_state,
            modal_body: rsx! {
                div { class: "input-group", style: "margin-top:10px",
                    span { class: "input-group-text", "Level" }
                    select {
                        class: "form-control",
                        oninput: move |e| {
                            state.write().log_level = LogApiLevel::from_str(e.value().as_str());
                        },

                        option { {LogApiLevel::Info.as_str()} }
                        option { {LogApiLevel::Warning.as_str()} }
                        option { {LogApiLevel::Error.as_str()} }
                        option { {LogApiLevel::FatalError.as_str()} }
                        option { {LogApiLevel::Debug.as_str()} }
                    }
                }

                div { class: "input-group", style: "margin-top:10px",
                    span { class: "input-group-text", "Application" }
                    input {
                        class: "form-control",
                        value: state.read().application.as_str(),
                        oninput: move |e| {
                            state.write().application = e.value();
                        },
                    }
                }

                div { class: "input-group", style: "margin-top:10px",
                    span { class: "input-group-text", "Marker" }
                    input {
                        class: "form-control",
                        value: state.read().marker.as_str(),
                        oninput: move |e| {
                            state.write().marker = e.value();
                        },
                    }
                }
            },
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let result = state.read().to_http_model();
                        dialog_state.set(DialogState::None);
                        on_ok.call(result);
                    },
                    "Add"
                }
            },
        }
    }
}

pub struct EventConfirmationState {
    pub log_level: LogApiLevel,
    pub application: String,
    pub marker: String,
}

impl EventConfirmationState {
    pub fn new() -> Self {
        Self {
            log_level: LogApiLevel::Error,
            application: "".to_string(),
            marker: "".to_string(),
        }
    }

    pub fn to_http_model(&self) -> IgnoreEventApiModel {
        IgnoreEventApiModel {
            level: self.log_level.clone(),
            application: self.application.clone(),
            marker: self.marker.clone(),
        }
    }
}
