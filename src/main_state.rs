use std::rc::Rc;

use crate::IgnoreEventApiModel;
#[derive(Clone)]
pub enum ActiveMenu {
    Dashboard,
    Logs,
    Settings(Option<Vec<Rc<IgnoreEventApiModel>>>),
}

impl ActiveMenu {
    pub fn unwrap_ignore_events(&self) -> Option<Vec<Rc<IgnoreEventApiModel>>> {
        match self {
            Self::Settings(value) => value.clone(),
            _ => {
                panic!("MainState::unwrap_ignore_events called on non-Settings variant")
            }
        }
    }
}

pub struct MainState {
    pub menu: ActiveMenu,
    pub active_env: Rc<String>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            menu: ActiveMenu::Logs,
            active_env: Rc::new("dev".to_string()),
        }
    }

    pub fn set_menu(&mut self, menu: ActiveMenu) {
        self.menu = menu;
    }
}
