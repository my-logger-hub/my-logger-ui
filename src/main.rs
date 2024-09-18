#![allow(non_snake_case)]

mod states;

mod date_key;

mod models;

#[cfg(feature = "server")]
use crate::app_ctx::AppContext;
use crate::{
    dialogs::{DialogState, RenderDialog},
    states::*,
    views::*,
};

use dioxus::prelude::*;
use models::LogPathDataModel;

#[cfg(feature = "server")]
mod app_ctx;
mod components;
#[cfg(feature = "server")]
mod grpc_client;
mod log_event_context_parser;
mod storage_settings;

#[cfg(feature = "server")]
mod settings_model;
mod views;

mod dialogs;

#[cfg(feature = "server")]
lazy_static::lazy_static! {
    pub static ref APP_CTX: AppContext = {
       AppContext::new()
    };
}

#[cfg(feature = "server")]
pub mod my_logger_grpc {
    tonic::include_proto!("my_logger");
}

const IGNORE_SINGLE_TIME_SUB_PATH: &str = "ignore-single-time";

// let cfg = dioxus::fullstack::Config::new().addr(([0, 0, 0, 0], 8080));

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/logs/:..data")]
    Logs { data: Vec<String> },

    #[route("/settings/:..data")]
    Settings { data: Vec<String> },
}

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(|| {
        rsx! {
            Router::<Route> {}
        }
    })
}

#[component]
fn Home() -> Element {
    use_context_provider(|| Signal::new(LocationState::Dashboard));

    App()
}

#[component]
fn Logs(data: Vec<String>) -> Element {
    use_context_provider(|| Signal::new(LocationState::Logs));

    if let Some(data) = data.get(0) {
        if let Some(model) = LogPathDataModel::from_base_64(data.as_str()) {
            crate::storage_settings::log_level::set(model.get_log_level());
            crate::storage_settings::search_line::set(&model.search_string);
            crate::storage_settings::ctx_search::set(model.is_ctx_search);
            crate::storage_settings::time_range::set_as_str(&model.time_range);
        }
    }

    //crate::storage_settings::clean_all();
    App()
}

#[component]
fn Settings(data: Vec<String>) -> Element {
    if let Some(data) = data.get(0) {
        if data == IGNORE_SINGLE_TIME_SUB_PATH {
            use_context_provider(|| Signal::new(LocationState::SettingsOneTimeIgnore));
            return App();
        }
    }
    use_context_provider(|| Signal::new(LocationState::SettingsIgnoreList));
    App()
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(DialogState::None));
    let mut main_state = consume_context::<Signal<MainState>>();

    let has_envs = { main_state.read().has_envs() };

    if has_envs {
        return rsx! {
            ActiveApp {}
        };
    }

    let resource = use_resource(|| get_envs());

    let data = resource.read_unchecked();

    match &*data {
        Some(data) => match data {
            Ok(result) => {
                let times = dioxus_utils::js::eval("new Date().getTimezoneOffset()");

                let time_zone = times.as_f64().unwrap() as i64;

                main_state
                    .write()
                    .set_environments(result.clone(), time_zone);
                return rsx! {
                    ActiveApp {}
                };
            }
            Err(err) => {
                let err = format!("Error loading environments. Err: {}", err);
                return rsx! {
                    {err}
                };
            }
        },

        None => {
            return rsx! { "Loading environments..." };
        }
    }
}

#[component]
fn ActiveApp() -> Element {
    let location_state_value = {
        let location_state = consume_context::<Signal<LocationState>>();
        let value = location_state.read();
        value.copy_state()
    };

    let right_panel = match location_state_value {
        LocationState::Dashboard => rsx! {
            RenderDashboard {}
        },
        LocationState::Logs => rsx! {
            RenderLogs {}
        },
        LocationState::SettingsIgnoreList => rsx! {
            RenderSettings {}
        },
        LocationState::SettingsOneTimeIgnore => rsx! {
            RenderSettings {}
        },
    };

    rsx! {
        div { id: "left-panel",
            div { style: "margin: 5px;", EnvsSelector {} }
            LeftPanel {}
        }
        div { id: "main-panel", {right_panel} }
        RenderDialog {}
    }
}

#[server]
pub async fn get_envs() -> Result<Vec<String>, ServerFnError> {
    let result = crate::APP_CTX
        .settings_reader
        .get_settings()
        .await
        .get_envs();

    Ok(result)
}
