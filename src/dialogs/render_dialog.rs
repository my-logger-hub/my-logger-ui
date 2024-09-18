use dioxus::prelude::*;

use crate::states::*;

use super::*;

#[component]
pub fn RenderDialog() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let mut header = "Dialog";

    let dialog_content = match dialog_state.read().clone() {
        DialogState::None => {
            return rsx! {
                div {}
            }
        }

        DialogState::AddIgnoreEvent(env) => {
            render_add_ignore_event_confirmation(env, &main_state, &dialog_state)
            //return rsx! { div {} };
        }

        DialogState::DeleteConfirmation { env, itm } => {
            render_delete_ignore_event_confirmation(env, itm, &main_state, &dialog_state)
        }

        DialogState::EditTimeRange {
            value,
            time_zone,
            on_change,
        } => {
            header = "Edit Time Range";
            edit_time_range_dialog(dialog_state, value, on_change, time_zone)
        }

        DialogState::EditOneTimeIgnoreEvent { itm, on_ok } => {
            header = "Edit OneTime Ignore Event";
            edit_one_time_ignore_event(dialog_state, itm, on_ok)
        }
    };

    rsx! {
        div { id: "dialog-pad",
            div { class: "modal", style: "display: block;",
                div { class: "modal-dialog",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h5 { class: "model-title", {header} }
                            button {
                                r#type: "button",
                                class: "btn-close",
                                onclick: move |_| {
                                    dialog_state.set(DialogState::None);
                                }
                            }
                        }
                        {dialog_content}
                    }
                }
            }
        }
    }
}
