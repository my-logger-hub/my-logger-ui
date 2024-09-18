use std::rc::Rc;

use dioxus::prelude::*;
use serde::*;

use crate::components::*;
use crate::{dialogs::DialogState, DataState, LogApiLevel, MainState};

#[component]
pub fn RenderIgnoreList() -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let mut main_state = consume_context::<Signal<MainState>>();

    let (env, data) = {
        let main_state_read_access = main_state.read();

        let env = main_state_read_access.get_selected_env();
        let data = main_state_read_access.ignore_events.clone();

        (env, data)
    };

    let value = match data {
        DataState::None => {
            main_state.write().ignore_events = DataState::Loading;
            load_ignore_events(env.clone(), main_state);

            return rsx! {
                h1 { "Loading" }
            };
        }

        DataState::Loading => {
            return rsx! {
                h1 { "Loading.." }
            };
        }

        DataState::Loaded(value) => value,
    };

    let table_content = value.into_iter().map(|itm| {
        let level = format!("{:?}", itm.level);
        let log_ball = render_log_ball(itm.level.clone());

        let env_cloned = env.clone();
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
                            dialog_state
                                .set(DialogState::DeleteConfirmation {
                                    env: env_cloned.clone(),
                                    itm: itm.clone(),
                                });
                        },
                        "Delete"
                    }
                }
            }
        }
    });

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
                            dialog_state.set(DialogState::AddIgnoreEvent(env.clone()));
                        },
                        "Add"
                    }
                }
            }

            {table_content}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IgnoreEventApiModel {
    pub level: LogApiLevel,
    pub application: String,
    pub marker: String,
}

fn load_ignore_events(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
        let result = get_ignore_events(env.to_string()).await.unwrap();
        let result = result.into_iter().map(|itm| Rc::new(itm)).collect();
        main_state.write().ignore_events = DataState::Loaded(result);
    });
}

#[server]
pub async fn get_ignore_events(env: String) -> Result<Vec<IgnoreEventApiModel>, ServerFnError> {
    use crate::my_logger_grpc::*;

    let response: Option<Vec<IgnoreEventGrpcModel>> = crate::APP_CTX
        .get_client(env.as_str())
        .await
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
