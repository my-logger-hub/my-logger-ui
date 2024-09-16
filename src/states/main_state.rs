use std::rc::Rc;

use dioxus_utils::js::WebLocalStorage;

use crate::{DashboardItem, IgnoreEventApiModel, LogApiItem};

pub const ENV_LOCAL_STORAGE_KEY: &str = "env";

pub struct MainState {
    pub storage: WebLocalStorage,
    pub envs: Option<Vec<Rc<String>>>,
    pub logs_data: Option<Rc<Vec<LogApiItem>>>,
    pub dashboard_data: Option<DashboardItem>,
    pub ignore_events: Option<Vec<Rc<IgnoreEventApiModel>>>,
}

impl MainState {
    pub fn new() -> Self {
        let storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();

        Self {
            envs: None,
            logs_data: None,
            dashboard_data: None,
            ignore_events: None,
            storage,
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();

        self.envs = Some(envs);
    }

    pub fn active_env_changed(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
        self.logs_data = None;
        self.dashboard_data = None;
        self.ignore_events = None;
    }

    pub fn get_selected_env(&self) -> Rc<String> {
        let env = self.storage.get(ENV_LOCAL_STORAGE_KEY).unwrap_or_default();

        for itm in self.envs.as_ref().unwrap() {
            if itm.as_str() == env {
                return itm.clone();
            }
        }

        self.envs.as_ref().unwrap().first().unwrap().clone()
    }

    pub fn set_logs_data(&mut self, value: Option<Vec<LogApiItem>>) {
        match value {
            Some(value) => {
                self.logs_data = Some(Rc::new(value));
            }
            None => {
                self.logs_data = None;
            }
        }
    }

    pub fn set_dashboard_data(&mut self, value: Option<DashboardItem>) {
        self.dashboard_data = value;
    }
}
