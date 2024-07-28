#![allow(non_snake_case)]

#[cfg(feature = "server")]
use crate::app_ctx::AppContext;
use crate::{
    dialogs::{DialogState, RenderDialog},
    main_state::MainState,
    views::*,
};

use dioxus::prelude::*;
use main_state::ActiveMenu;

#[cfg(feature = "server")]
mod app_ctx;
#[cfg(feature = "server")]
mod grpc_client;
mod log_event_context_parser;
mod main_state;
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

    LaunchBuilder::fullstack().with_cfg(cfg).launch(app)
}

fn app() -> Element {
    use_context_provider(|| Signal::new(MainState::new()));
    use_context_provider(|| Signal::new(DialogState::None));

    let main_state = consume_context::<Signal<MainState>>();

    let main_state_value = main_state.read();

    let right_panel = match main_state_value.menu.clone() {
        ActiveMenu::Dashboard => rsx! {
            RenderDashboard {}
        },
        ActiveMenu::Logs => rsx! {
            RenderLogs {}
        },
        ActiveMenu::Settings(_) => rsx! {
            RenderSettings {}
        },
    };

    rsx! {
        LeftPanel {}
        div { id: "main-panel", {right_panel} }
        RenderDialog {}
    }
}
