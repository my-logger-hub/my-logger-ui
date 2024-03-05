use std::rc::Rc;

use dioxus::prelude::*;
use serde::*;

use crate::{dialogs::DialogState, main_state::MainState, render_log_ball, LogApiLevel};

pub fn render_settings() -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let main_state = consume_context::<Signal<MainState>>();

    let ignore_events = main_state.read().unwrap_ignore_events();

    let content = match ignore_events {
        Some(value) => {
            let table_content: Vec<_> = value
                .into_iter()
                .map(|itm| {
                    let level = format!("{:?}", itm.level);
                    let log_ball = render_log_ball(itm.level.clone());

                    rsx! {
                        tr {
                            td { {log_ball} }
                            td { "{level}" }
                            td { "{itm.application}" }
                            td { "{itm.marker}" }
                            td {
                                a {
                                    class: "btn btn-sm btn-danger",
                                    style: "padding:2px 6px",

                                    onclick: move |_| {
                                        dialog_state.set(DialogState::DeleteConfirmation(itm.clone()));
                                    },
                                    "Delete"
                                }
                            }
                        }
                    }
                })
                .collect();

            rsx! {
                table { class: "table table-striped",
                    tr {
                        th { style: "width:25px" }
                        th { "Level" }
                        th { "Application" }
                        th { "Marker" }
                        th {
                            button {
                                class: "btn btn-sm btn-primary",
                                style: "padding:2px 6px",
                                onclick: move |_| {
                                    dialog_state.set(DialogState::AddIgnoreEvent);
                                },
                                "Add"
                            }
                        }
                    }

                    {table_content.into_iter()}
                }
            }
        }
        None => {
            load_ignore_events(&main_state);
            return rsx! { h1 { "Loading" } };
        }
    };

    rsx! {
        ul { class: "nav",
            li {
                style: "box-shadow: 1px 1px 1px lightgray; width:100%",
                class: "nav-item",
                a { class: "nav-link active", "Ignore list" }
            }
        }

        {content}
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IgnoreEventApiModel {
    pub level: LogApiLevel,
    pub application: String,
    pub marker: String,
}

fn load_ignore_events(main_state: &Signal<MainState>) {
    let mut main_state = main_state.to_owned();

    spawn(async move {
        let result = get_ignore_events().await.unwrap();

        let result = result.into_iter().map(|itm| Rc::new(itm)).collect();

        main_state.set(MainState::Settings(Some(result)));
    });
}

#[server]
pub async fn get_ignore_events() -> Result<Vec<IgnoreEventApiModel>, ServerFnError> {
    use crate::my_logger_grpc::*;

    let response: Option<Vec<IgnoreEventGrpcModel>> = crate::APP_CTX
        .grpc_client
        .get_ignore_events(())
        .await
        .unwrap();

    let result = match response {
        Some(response) => response
            .into_iter()
            .map(|itm: IgnoreEventGrpcModel| IgnoreEventApiModel {
                level: (&itm.level()).into(),
                application: itm.application,
                marker: itm.marker,
            })
            .collect(),
        None => vec![],
    };

    Ok(result)
}

/*

         button {
                                class: "btn btn-sm btn-primary",
                                style: "padding:2px 6px",
                                onclick: move |_| {
                                    dialog_state.set(DialogState::AddIgnoreEvent);
                                },
                                "Add"
                            }
*/
