use std::rc::Rc;

use crate::IgnoreEventApiModel;

#[derive(Clone)]
pub enum DialogState {
    None,
    AddIgnoreEvent(Rc<String>),
    DeleteConfirmation {
        env: Rc<String>,
        itm: Rc<IgnoreEventApiModel>,
    },
}
