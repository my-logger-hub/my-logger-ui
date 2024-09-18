use std::rc::Rc;

use dioxus::prelude::EventHandler;

use crate::{IgnoreEventApiModel, OneTimeIgnoreHttpModel};

use super::TimeRange;

#[derive(Clone)]
pub enum DialogState {
    None,
    AddIgnoreEvent(Rc<String>),
    DeleteConfirmation {
        env: Rc<String>,
        itm: Rc<IgnoreEventApiModel>,
    },
    EditTimeRange {
        value: TimeRange,
        time_zone: i64,
        on_change: EventHandler<TimeRange>,
    },

    EditOneTimeIgnoreEvent {
        itm: Rc<OneTimeIgnoreHttpModel>,
        on_ok: EventHandler<OneTimeIgnoreHttpModel>,
    },
}
