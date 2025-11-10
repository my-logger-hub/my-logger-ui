use std::rc::Rc;

use dioxus::prelude::*;

use crate::components::*;

use crate::{dialogs::DialogState, MainState};

use dioxus_utils::*;


#[component]
pub fn RenderIgnoreList() -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let mut main_state = consume_context::<Signal<MainState>>();
    let main_state_read_access = main_state.read();

    let env = main_state_read_access.get_selected_env();

    let value = match main_state_read_access.ignore_events.as_ref() {
        RenderState::None => {
  
            load_ignore_events(env.clone(), main_state);

            return rsx! {
                h1 { "Loading" }
            };
        }

        RenderState::Loading => {
            return rsx! {
                h1 { "Loading.." }
            };
        }

        RenderState::Loaded(value) => value,
        RenderState::Error(err) => return rsx! { "Error loading data: {err}" },
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
                                            let _ = crate::api::ignore_events::delete_ignore_event(
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
                                                    let _ = crate::api::ignore_events::add_ignore_event(
                                                            env.to_string(),
                                                            itm,
                                                        )
                                                        .await;
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


fn load_ignore_events(env: Rc<String>, mut main_state: Signal<MainState>) {
    spawn(async move {
        main_state.write().ignore_events.set_loading();
        let result = crate::api::ignore_events::get_ignore_events(env.to_string()).await;

        match result{
            Ok(result) => {
                let result = result.into_iter().map(|itm| Rc::new(itm)).collect();
                main_state.write().ignore_events.set_loaded(result);
            }
            Err(err) => {
                main_state.write().ignore_events.set_error(err.to_string());
            }
        }

    });
}

