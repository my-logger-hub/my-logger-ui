use std::rc::Rc;

use crate::IgnoreEventApiModel;

#[derive(Clone)]
pub enum DialogState {
    None,
    DeleteConfirmation(Rc<IgnoreEventApiModel>),
}
