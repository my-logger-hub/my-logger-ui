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
            return render_add_ignore_event_confirmation(dialog_state, on_ok);
            //return rsx! { div {} };
        }

        DialogState::Confirmation { text, on_ok } => {
            return render_confirmation(text, dialog_state, on_ok)
        }

        DialogState::EditTimeRange {
            value,
            time_zone,
            on_change,
        } => return edit_time_range_dialog(dialog_state, value, on_change, time_zone),

        DialogState::EditOneTimeIgnoreEvent { itm, on_ok } => {
            return edit_one_time_ignore_event(dialog_state, itm, on_ok)
        }
    }
}
