use std::rc::Rc;

use crate::IgnoreEventApiModel;

#[derive(Clone)]
pub enum MainState {
    Dashboard,
    Logs,
    Settings(Option<Vec<Rc<IgnoreEventApiModel>>>),
}

impl MainState {
    pub fn unwrap_ignore_events(&self) -> Option<Vec<Rc<IgnoreEventApiModel>>> {
        match self {
            MainState::Settings(value) => value.clone(),
            _ => {
                panic!("MainState::unwrap_ignore_events called on non-Settings variant")
            }
        }
    }
}
