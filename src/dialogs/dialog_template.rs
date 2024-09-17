use dioxus::prelude::*;

use crate::dialogs::DialogState;
#[component]
pub fn DialogTemplate(
    dialog_state: Signal<DialogState>,
    modal_body: Element,
    ok_button: Element,
) -> Element {
    rsx! {
        div { class: "modal-body", {modal_body} }
        div { class: "modal-footer",
            div { class: "btn-group",
                {ok_button},
                button {
                    class: "btn btn-outline-dark",
                    onclick: move |_| { dialog_state.set(DialogState::None) },
                    "Cancel"
                }
            }
        }
    }
}
