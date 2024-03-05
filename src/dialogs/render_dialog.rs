use dioxus::prelude::*;

use crate::main_state::MainState;

use super::*;

#[component]
pub fn RenderDialog() -> Element {
    let main_state = consume_context::<Signal<MainState>>();
    let mut dialog_state = consume_context::<Signal<DialogState>>();

    let dialog_content = match dialog_state.read().clone() {
        DialogState::None => return rsx! { div {} },

        DialogState::AddIgnoreEvent => {
            render_add_ignore_event_confirmation(&main_state, &dialog_state)
            //return rsx! { div {} };
        }

        DialogState::DeleteConfirmation(itm) => {
            render_delete_ignore_event_confirmation(itm, &main_state, &dialog_state)
        }
    };

    rsx! {
        div { id: "dialog-pad",
            div { class: "modal", style: "display: block;",
                div { class: "modal-dialog",
                    div { class: "modal-content",
                        div { class: "modal-header",
                            h5 { class: "model-title", "Header" }
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
