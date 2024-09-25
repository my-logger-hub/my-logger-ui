use std::rc::Rc;

use dioxus::prelude::EventHandler;

use crate::{IgnoreEventApiModel, OneTimeIgnoreHttpModel};

use crate::models::*;

#[derive(Clone)]
pub enum DialogState {
    None,
    AddIgnoreEvent {
        on_ok: EventHandler<IgnoreEventApiModel>,
    },
    Confirmation {
        text: Rc<String>,
        on_ok: EventHandler<()>,
    },
    EditTimeRange {
        value: TimeRange,
        time_zone: TimeZone,
        on_change: EventHandler<TimeRange>,
    },

    EditOneTimeIgnoreEvent {
        itm: Rc<OneTimeIgnoreHttpModel>,
        on_ok: EventHandler<OneTimeIgnoreHttpModel>,
    },
}
