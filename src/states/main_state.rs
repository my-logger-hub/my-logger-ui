use std::rc::Rc;

use dioxus_utils::js::WebLocalStorage;

use crate::{DashboardItem, IgnoreEventApiModel, LogApiItem, OneTimeIgnoreHttpModel};

pub const ENV_LOCAL_STORAGE_KEY: &str = "env";
use super::DataState;

pub struct MainState {
    pub storage: WebLocalStorage,
    pub envs: Option<Vec<Rc<String>>>,
    pub logs_data: DataState<Vec<Rc<LogApiItem>>>,
    pub dashboard_data: DataState<DashboardItem>,
    pub ignore_events: DataState<Vec<Rc<IgnoreEventApiModel>>>,
    pub one_time_ignore_events: DataState<Vec<Rc<OneTimeIgnoreHttpModel>>>,
    pub time_zone: i64,
}

impl MainState {
    pub fn new() -> Self {
        let storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();

        Self {
            envs: None,
            logs_data: DataState::None,
            dashboard_data: DataState::None,
            ignore_events: DataState::None,
            one_time_ignore_events: DataState::None,
            storage,
            time_zone: 0,
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>, time_zone: i64) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();

        self.envs = Some(envs);
        self.time_zone = time_zone;
    }

    pub fn active_env_changed(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
        self.reset_data();
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

    pub fn reset_data(&mut self) {
        self.logs_data = DataState::None;
        self.dashboard_data = DataState::None;
        self.ignore_events = DataState::None;
        self.one_time_ignore_events = DataState::None;
    }
}
