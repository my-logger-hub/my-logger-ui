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
        DataState::Error(err) => return rsx! { "Error loading data: {err}" },
    };

    let table_content = value.into_iter().map(|itm| {
        let level = format!("{:?}", itm.level);
        let log_ball = render_log_ball(itm.level.clone());

        let phrase: Rc<String> = format!(
            "Are you sure you want to delete the ignore event for {:?} for application {} with marker{} ?",
            itm.level,
            itm.application,
            itm.marker
        ).into();

        let itm_to_delete = itm.clone();
        let env_delete = env.clone();
        
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
                            let phrase = phrase.clone();
                            let env_delete = env_delete.clone();
                            let itm_to_delete = itm_to_delete.clone();
                            dialog_state
                                .set(DialogState::Confirmation {
                                    text: phrase,
                                    on_ok: EventHandler::new(move |_| {
                                        let env = env_delete.clone();
                                        let itm_to_delete = itm_to_delete.clone();
                                        spawn(async move {
                                            let _ = delete_ignore_event(
                                                    env.to_string(),
                                                    itm_to_delete.as_ref().clone(),
                                                )
                                                .await;
                                            main_state.write().reset_data();
                                            dialog_state.set(DialogState::None)
                                        });
                                    }),
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
            thead {
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
                                let env = env.clone();
                                dialog_state
                                    .set(DialogState::AddIgnoreEvent {
                                        on_ok: EventHandler::new(move |itm| {
                                            let env = env.clone();
                                            spawn(async move {
                                                let env = env.clone();
                                                spawn(async move {
                                                    let _ = add_ignore_event(env.to_string(), itm).await;
                                                    main_state.write().reset_data();
                                                    dialog_state.set(DialogState::None);
                                                });
                                                main_state.write().reset_data();
                                            });
                                        }),
                                    });
                            },
                            "Add"
                        }
                    }
                }
            }

            tbody { {table_content} }
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
        let result = get_ignore_events(env.to_string()).await;

        match result{
            Ok(result) => {
                let result = result.into_iter().map(|itm| Rc::new(itm)).collect();
                main_state.write().ignore_events = DataState::Loaded(result);
            }
            Err(err) => {
                main_state.write().ignore_events = DataState::Error(err.to_string());
            }
        }

    });
}

#[server]
pub async fn get_ignore_events(env: String) -> Result<Vec<IgnoreEventApiModel>, ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let response: Option<Vec<IgnoreEventGrpcModel>> = crate::server::APP_CTX
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

#[server]
pub async fn add_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let level: LogLevelGrpcModel = (&event.level).into();

    crate::server::APP_CTX
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

#[server]
pub async fn delete_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let level: LogLevelGrpcModel = (&event.level).into();

    crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .delete_ignore_event(DeleteIgnoreEventGrpcRequest {
            level: level as i32,
            application: event.application,
            marker: event.marker,
        })
        .await
        .unwrap();

    Ok(())
}
