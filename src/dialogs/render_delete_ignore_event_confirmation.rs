use crate::IgnoreEventApiModel;
use dioxus::prelude::*;
use std::rc::Rc;

use super::DialogState;

pub fn render_delete_ignore_event_confirmation(
    model: Rc<IgnoreEventApiModel>,
    dialog_state: &Signal<DialogState>,
) -> Element {
    let mut dialog_state = dialog_state.to_owned();
    let phrase = format!(
        "Are you sure you want to delete the ignore event for {:?} for application {} with marker{} ?",
        model.level,
        model.application,
        model.marker
    );
    rsx! {
        div { class: "modal-body",
            div { {phrase} }
        }
        div { class: "modal-footer",
            div { class: "btn-group",
                button {
                    class: "btn btn-danger",
                    onclick: move |_| {
                        let itm_to_request = model.as_ref().clone();
                        spawn(async move {
                            delete_ignore_event(itm_to_request).await.unwrap();
                            dialog_state.set(DialogState::None)
                        });
                    },
                    "Delete"
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
pub async fn delete_ignore_event(event: IgnoreEventApiModel) -> Result<(), ServerFnError> {
    use crate::my_logger_grpc::*;

    let level: LogLevelGrpcModel = (&event.level).into();

    crate::APP_CTX
        .grpc_client
        .delete_ignore_event(DeleteIgnoreEventGrpcRequest {
            level: level as i32,
            application: event.application,
            marker: event.marker,
        })
        .await
        .unwrap();

    Ok(())
}
