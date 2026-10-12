use std::rc::Rc;

use dioxus_utils::{js::WebLocalStorage, DataState};

use crate::storage_settings::selected_time_zone::SelectedTimeZone;

use crate::models::*;

pub const ENV_LOCAL_STORAGE_KEY: &str = "env";

pub struct MainState {
    pub storage: WebLocalStorage,
    pub envs: Option<Vec<Rc<String>>>,
    /// Version of the app as the server reports it - the UI is built into the same release
    pub app_version: String,
    pub logs_data: DataState<Vec<Rc<LogApiItem>>>,
    pub dashboard_data: DataState<DashboardItem>,
    pub ignore_events: DataState<Vec<Rc<IgnoreEventApiModel>>>,
    pub one_time_ignore_events: DataState<Vec<Rc<OneTimeIgnoreHttpModel>>>,
    pub selected_time_zone: SelectedTimeZone,
    pub server_settings: DataState<Rc<ServerInfoHttpModel>>,
    pub product_scopes: DataState<Rc<ProductScopesHttpModel>>,
    /// The product scope the dashboard shows - the one of the local storage, which the logs
    /// are read from as well. Empty - the default one of the env
    pub product_scope: String,
    time_zone: TimeZone,
}

impl Default for MainState {
    fn default() -> Self {
        let storage = dioxus_utils::js::GlobalAppSettings::get_local_storage();

        let selected_time_zone = crate::storage_settings::selected_time_zone::get();
        Self {
            envs: None,
            app_version: String::new(),
            logs_data: DataState::default(),
            dashboard_data: DataState::default(),
            ignore_events: DataState::default(),
            one_time_ignore_events: DataState::default(),
            server_settings: DataState::default(),
            product_scopes: DataState::default(),
            product_scope: crate::storage_settings::product_scope::get(),
            storage,
            time_zone: TimeZone::default(),
            selected_time_zone,
        }
    }
}

impl MainState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has_envs(&self) -> bool {
        self.envs.is_some()
    }

    pub fn set_app_info(&mut self, app_info: AppInfoHttpModel, time_zone: TimeZone) {
        let envs: Vec<Rc<String>> = app_info.envs.into_iter().map(Rc::new).collect();

        self.envs = Some(envs);
        self.app_version = app_info.version;
        self.time_zone = time_zone;
    }

    pub fn get_selected_timezone(&self) -> TimeZone {
        match self.selected_time_zone {
            SelectedTimeZone::UtcZero => TimeZone::create_utc_zero(),
            SelectedTimeZone::LocalTime => self.time_zone,
        }
    }

    pub fn get_local_timezone(&self) -> TimeZone {
        self.time_zone
    }

    pub fn active_env_changed(&mut self, value: &str) {
        dioxus_utils::js::GlobalAppSettings::get_local_storage().set(ENV_LOCAL_STORAGE_KEY, value);
        self.reset_data();
        self.server_settings.reset();
        // The scopes of one env are not the ones of another
        self.product_scopes.reset();
        self.set_product_scope("");
    }

    pub fn set_product_scope(&mut self, product_scope: &str) {
        crate::storage_settings::product_scope::set(product_scope);
        self.product_scope = product_scope.to_string();
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
        self.logs_data.reset();
        self.dashboard_data.reset();
        self.ignore_events.reset();
        self.one_time_ignore_events.reset();
    }
}
