use std::sync::Arc;

use my_settings_reader::SettingsReader;
use rust_extensions::AppStates;

use crate::{grpc_client::LoggerGrpcClients, settings::SettingsModel};

use super::UiUrl;

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const SETTINGS_FILE_NAME: &str = "~/.my-logger";

pub struct AppContext {
    pub logger_grpc_clients: LoggerGrpcClients,
    pub ui_url: UiUrl,
    pub states: Arc<AppStates>,
    pub settings_reader: SettingsReader<SettingsModel>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            logger_grpc_clients: LoggerGrpcClients::new(),
            ui_url: UiUrl::new(),
            states: Arc::new(AppStates::create_initialized()),
            settings_reader: SettingsReader::new(SETTINGS_FILE_NAME),
        }
    }
}
