use std::rc::Rc;

use crate::{DashboardItem, IgnoreEventApiModel, LogApiItem};

pub struct MainState {
    pub selected_env: Rc<String>,
    pub envs: Option<Vec<Rc<String>>>,
    pub logs_data: Option<Rc<Vec<LogApiItem>>>,
    pub dashboard_data: Option<DashboardItem>,
    pub ignore_events: Option<Vec<Rc<IgnoreEventApiModel>>>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            selected_env: Rc::new("".to_string()),
            envs: None,
            logs_data: None,
            dashboard_data: None,
            ignore_events: None,
        }
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_environments(&mut self, envs: Vec<String>) {
        let envs: Vec<Rc<String>> = envs.into_iter().map(Rc::new).collect();
        self.selected_env = envs[0].clone();
        self.envs = Some(envs);
    }

    pub fn set_active_env(&mut self, env: &str) {
        let found_value = self
            .envs
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|itm| itm.as_str() == env);

        if let Some(found_value) = found_value {
            self.selected_env = found_value.clone();
            self.logs_data = None;
            self.dashboard_data = None;
            self.ignore_events = None;
        }
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
