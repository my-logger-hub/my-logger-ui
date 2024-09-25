use std::{collections::HashMap, rc::Rc};

use dioxus::prelude::*;

use serde::*;

use crate::{dialogs::DialogState, *};
use components::*;

#[component]
pub fn RenderOneTimeIgnoreList() -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();

    let value = match main_state_read_access.one_time_ignore_events.clone() {
        DataState::None => {
            drop(main_state_read_access);
            main_state.write().one_time_ignore_events = DataState::Loading;
            load_from_db(env.clone());
            return rsx! {
                h1 { "Loading" }
            };
        }

        DataState::Loading => {
            return rsx! {
                h1 { "Loading" }
            };
        }

        DataState::Loaded(value) => value,
    };

    let table_content = value.into_iter().map(|itm| {
        let levels = itm.levels.iter().map(|level| {
            let log_ball = render_log_ball(level.clone());
            rsx! {
                tr {
                    td { {log_ball} }
                    td { {level.as_str()} }
                }
            }
        });

        
        
        let ctx_matches = if let Some(ctx) = itm.ctx_match.as_ref(){
            let items = ctx.iter().map(|(key, value)| {
                rsx! {
                    div { "{key.as_str()}:{value.as_str()}" }
                }
            });

            rsx!{
                {items}
            }
        }else{
            rsx! {
                div {}
            }
        };
        

        let env = env.clone();
        let env_to_delete = env.clone();
        let id_to_delete = Rc::new(itm.id.clone());
        rsx! {
            tr {
                td { {levels} }

                td { {itm.message_match.as_str()} }

                td { {ctx_matches} }
                td { {itm.skip_amount.to_string()} }
                td { {itm.minutes_to_wait.to_string()} }
                td {

                    div { class: "btn-group",
                        button {
                            class: "btn btn-primary btn-sm",
                            style: "padding:2px 6px",
                            onclick: move |_| {
                                let env = env.clone();
                                let itm = itm.clone();
                                dialog_state
                                    .set(DialogState::EditOneTimeIgnoreEvent {
                                        itm: itm,
                                        on_ok: EventHandler::new(move |item_to_save| {
                                            let env = env.clone();
                                            spawn(async move {
                                                save_one_time_ignore_event(env.to_string(), item_to_save)
                                                    .await
                                                    .unwrap();
                                                main_state.write().reset_data();
                                            });
                                        }),
                                    });
                            },
                            "Edit"
                        }
                        button {
                            class: "btn btn-primary btn-sm",
                            style: "padding:2px 6px",
                            onclick: move |_| {
                                let env_to_delete = env_to_delete.clone();
                                let id_to_delete = id_to_delete.clone();
                                dialog_state
                                    .set(DialogState::Confirmation {
                                        text: "Please confirm that you want to delete this item"
                                            .to_string()
                                            .into(),
                                        on_ok: EventHandler::new(move |_| {
                                            let env_to_delete = env_to_delete.clone();
                                            let id_to_delete = id_to_delete.clone();
                                            spawn(async move {
                                                delete_one_time_ignore_event(
                                                        env_to_delete.to_string(),
                                                        id_to_delete.to_string(),
                                                    )
                                                    .await
                                                    .unwrap();
                                                main_state.write().reset_data();
                                            });
                                        }),
                                    });
                            },
                            "Delete"
                        }
                    }
                }
            }
        }
    });

    rsx! {
        table { class: "table table-striped",
            tr {
                th { "Levels" }
                th { "Message match" }
                th { "Context Match" }
                th { "Skip amount" }
                th { "Minutes to wait" }
                th {
                    button {
                        class: "btn btn-primary btn-sm",
                        style: "padding:2px 6px",
                        onclick: move |_| {
                            let env = env.clone();
                            dialog_state
                                .set(DialogState::EditOneTimeIgnoreEvent {
                                    itm: OneTimeIgnoreHttpModel::default().into(),
                                    on_ok: EventHandler::new(move |item_to_save| {
                                        let env = env.clone();
                                        spawn(async move {
                                            save_one_time_ignore_event(env.to_string(), item_to_save)
                                                .await
                                                .unwrap();
                                            consume_context::<Signal<MainState>>().write().reset_data();
                                        });
                                    }),
                                });
                        },
                        "+"
                    }
                }
            }

            {table_content}
        }
    }
}

fn load_from_db(env: Rc<String>) {
    spawn(async move {
        let mut main_state = consume_context::<Signal<MainState>>();
        let result = get_one_time_ignore_events(env.to_string()).await.unwrap();
        let result = result.into_iter().map(|itm| Rc::new(itm)).collect();
        main_state.write().one_time_ignore_events = DataState::Loaded(result);
    });
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OneTimeIgnoreHttpModel {
    pub id: String,
    pub levels: Vec<LogApiLevel>,
    pub message_match: String,
    pub ctx_match: Option<HashMap<String, String>>,
    pub skip_amount: u64,
    pub minutes_to_wait: u64,
}

impl Default for OneTimeIgnoreHttpModel {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            levels: vec![],
            message_match: "".to_string(),
            ctx_match: None,
            skip_amount: 0,
            minutes_to_wait: 0,
        }
    }
}

#[server]
pub async fn get_one_time_ignore_events(
    env: String,
) -> Result<Vec<OneTimeIgnoreHttpModel>, ServerFnError> {
    let response = crate::APP_CTX
        .get_client(env.as_str())
        .await
        .get_ignore_single_events(())
        .await
        .unwrap();

    let result = match response {
        Some(response) => response
            .into_iter()
            .map(|itm| {
                let levels = itm.levels().map(|itm| itm.into()).collect();

                let ctx_match = if itm.context_match.len()>0{
                    Some(itm
                    .context_match
                    .iter()
                    .map(|itm| (itm.key.to_string(), itm.value.to_string()))
                    .collect())
                }  else{
                    None
                };

                OneTimeIgnoreHttpModel {
                    id: itm.id,
                    levels,
                    message_match: itm.message_match,
                    ctx_match,
                    skip_amount: itm.skip_amount,
                    minutes_to_wait: itm.minutes_to_wait,
                }
            })
            .collect(),
        None => vec![],
    };

    Ok(result)
}

#[server]
async fn save_one_time_ignore_event(
    env: String,
    itm: OneTimeIgnoreHttpModel,
) -> Result<(), ServerFnError> {
    use my_logger_grpc::*;

    crate::APP_CTX
        .get_client(env.as_str())
        .await
        .set_ignore_single_event(IgnoreSingleEventGrpcModel {
            id: itm.id,
            levels: itm
                .levels
                .iter()
                .map(|itm| {
                    let log_level: LogLevelGrpcModel = itm.into();
                    let result: i32 = log_level.into();
                    result
                })
                .collect(),
            message_match: itm.message_match,
            context_match: if let Some(ctx_match) = itm.ctx_match.as_ref(){
                ctx_match
                .iter()
                .map(|(k, v)| LogEventContext {
                    key: k.to_string(),
                    value: v.to_string(),
                })
                .collect()
            }else{
                vec![]
            },
            skip_amount: itm.skip_amount,
            minutes_to_wait: itm.minutes_to_wait,
        })
        .await
        .unwrap();


    Ok(())
}

#[server]
async fn delete_one_time_ignore_event(env: String, id: String) -> Result<(), ServerFnError> {
    use my_logger_grpc::*;
    crate::APP_CTX
        .get_client(env.as_str())
        .await
        .delete_ignore_single_event(DeleteIgnoreSingleEventGrpcRequest { id })
        .await
        .unwrap();

    Ok(())
}
