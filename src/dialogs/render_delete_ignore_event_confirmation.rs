use crate::{
    main_state::{ActiveMenu, MainState},
    IgnoreEventApiModel,
};
use dioxus::prelude::*;
use std::rc::Rc;

use super::DialogState;

pub fn render_delete_ignore_event_confirmation(
    env: Rc<String>,
    model: Rc<IgnoreEventApiModel>,
    main_state: &Signal<MainState>,
    dialog_state: &Signal<DialogState>,
) -> Element {
    let mut dialog_state = dialog_state.to_owned();
    let mut main_state = main_state.to_owned();
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
                        let env = env.clone();
                        spawn(async move {
                            delete_ignore_event(env.to_string(), itm_to_request).await.unwrap();
                            main_state.write().set_menu(ActiveMenu::Settings(None));
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
pub async fn delete_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::my_logger_grpc::*;

    let level: LogLevelGrpcModel = (&event.level).into();

    crate::APP_CTX
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
