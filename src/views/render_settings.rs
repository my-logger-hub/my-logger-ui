use std::rc::Rc;

use dioxus::prelude::*;
use serde::*;

use crate::{dialogs::DialogState, render_log_ball, LogApiLevel};

pub fn render_settings() -> Element {
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let widget_state: Signal<Option<Vec<Rc<IgnoreEventApiModel>>>, _> = use_signal(|| None);

    let read = widget_state.read();

    let content = match read.as_ref() {
        Some(value) => {
            let table_content: Vec<_> = value
                .iter()
                .map(|itm: &Rc<IgnoreEventApiModel>| {
                    let level = format!("{:?}", itm.level);
                    let log_ball = render_log_ball(itm.level.clone());

                    let delete_item: Rc<IgnoreEventApiModel> = itm.clone();
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
                                        dialog_state.set(DialogState::DeleteConfirmation(delete_item.clone()));
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
                        th {}
                    }

                    {table_content.into_iter()}
                }
            }
        }
        None => {
            load_ignore_events(&widget_state);
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

fn load_ignore_events<'s>(
    widget_state: &Signal<Option<Vec<Rc<IgnoreEventApiModel>>>, UnsyncStorage>,
) {
    let mut widget_state = widget_state.to_owned();

    spawn(async move {
        let result = get_ignore_events().await.unwrap();

        let result = result.into_iter().map(|itm| Rc::new(itm)).collect();

        widget_state.set(Some(result));
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
