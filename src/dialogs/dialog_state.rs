use std::rc::Rc;

use crate::IgnoreEventApiModel;

#[derive(Clone)]
pub enum DialogState {
    None,
    AddIgnoreEvent,
    DeleteConfirmation(Rc<IgnoreEventApiModel>),
}
