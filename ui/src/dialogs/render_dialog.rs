use dioxus::prelude::*;

use super::*;

#[component]
pub fn RenderDialog() -> Element {
    let dialog_state = consume_context::<Signal<DialogState>>();

    let dialog_state_value = { dialog_state.read().clone() };
    match dialog_state_value {
        DialogState::None => {
            return rsx! {
                div {}
            }
        }

        DialogState::AddIgnoreEvent { on_ok } => {
            return rsx! {
                RenderAddIgnoreEvent { on_ok }
            };
            //return rsx! { div {} };
        }

        DialogState::Confirmation { text, on_ok } => {
            return rsx! {
                RenderConfirmation { text, on_ok }
            }
        }

        DialogState::EditTimeRange {
            value,
            time_zone,
            on_change,
        } => {
            return rsx! {
                EditTimeRangeDialog { value, on_change, time_zone }
            }
        }

        DialogState::EditOneTimeIgnoreEvent { itm, on_ok } => {
            return rsx! {
                EditOneTimeIgnoreDialog { value: itm, on_ok }
            }
        }
    }
}
