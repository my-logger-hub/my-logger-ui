#![allow(non_snake_case)]

mod states;

#[cfg(feature = "server")]
use crate::app_ctx::AppContext;
use crate::{
    dialogs::{DialogState, RenderDialog},
    states::*,
    views::*,
};

use dioxus::prelude::*;

#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod grpc_client;
mod log_event_context_parser;

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

// let cfg = dioxus::fullstack::Config::new().addr(([0, 0, 0, 0], 8080));

fn main() {
    let cfg = dioxus::fullstack::Config::new();

    #[cfg(feature = "server")]
    let cfg = cfg.addr(([0, 0, 0, 0], 9001));

    LaunchBuilder::fullstack().with_cfg(cfg).launch(App)
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(LocationState::Dashboard));
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
                main_state.write().set_environments(result.clone());
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
        LocationState::Settings => rsx! {
            RenderSettings {}
        },

        LocationState::IgnoreSingleEvents => rsx! {
            div {}
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
