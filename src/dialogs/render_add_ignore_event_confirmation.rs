use std::rc::Rc;

use crate::{states::*, IgnoreEventApiModel, LogApiLevel};
use dioxus::prelude::*;

use super::DialogState;

pub struct LogLevel(String);

impl LogLevel {
    pub fn to_api_model(&self) -> LogApiLevel {
        match self.0.as_str() {
            "Info" => LogApiLevel::Info,
            "Warning" => LogApiLevel::Warning,
            "Error" => LogApiLevel::Error,
            "FatalError" => LogApiLevel::FatalError,
            _ => LogApiLevel::Debug,
        }
    }
}

pub struct Application(String);
impl Application {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
pub struct Marker(String);

impl Marker {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub fn render_add_ignore_event_confirmation(
    env: Rc<String>,
    main_state: &Signal<MainState>,
    dialog_state: &Signal<DialogState>,
) -> Element {
    let mut dialog_state = dialog_state.to_owned();
    let mut main_state = main_state.to_owned();

    let mut log_level = use_signal(|| LogLevel("".to_string()));

    let mut application = use_signal(|| Application("".to_string()));

    let mut marker = use_signal(|| Marker("".to_string()));

    rsx! {
        div { class: "modal-body",
            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "Level" }
                select {
                    class: "form-control",
                    oninput: move |e| { log_level.set(LogLevel(e.value())) },

                    option { "Info" }
                    option { "Warning" }
                    option { "Error" }
                    option { "FatalError" }
                    option { "Debug" }
                }
            }

            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "Application" }
                input {
                    class: "form-control",
                    oninput: move |e| { application.set(Application(e.value())) }
                }
            }

            div { class: "input-group", style: "margin-top:10px",
                span { class: "input-group-text", "Marker" }
                input {
                    class: "form-control",
                    oninput: move |e| { marker.set(Marker(e.value())) }
                }
            }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        let level = log_level.read().to_api_model();
                        let application = application.read().to_string();
                        let marker = marker.read().to_string();
                        let env = env.clone();
                        spawn(async move {
                            add_ignore_event(
                                    env.to_string(),
                                    IgnoreEventApiModel {
                                        level,
                                        application,
                                        marker,
                                    },
                                )
                                .await
                                .unwrap();
                            main_state.write().reset_data();
                            dialog_state.set(DialogState::None);
                        });
                    },
                    "Add"
                }
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| { dialog_state.set(DialogState::None) },
                    "Cancel"
                }
            }
        }
    }
}

#[server]
pub async fn add_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::my_logger_grpc::*;

    let level: LogLevelGrpcModel = (&event.level).into();

    crate::APP_CTX
        .get_client(env.as_str())
        .await
        .set_ignore_event(IgnoreEventGrpcModel {
            level: level as i32,
            application: event.application,
            marker: event.marker,
        })
        .await
        .unwrap();

    Ok(())
}
