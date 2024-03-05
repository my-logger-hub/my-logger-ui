use std::sync::Arc;

use crate::{grpc_client::MyLoggerGrpcClient, settings_reader::SettingsReader};

pub struct AppContext {
    pub grpc_client: Arc<MyLoggerGrpcClient>,
}

impl AppContext {
    pub fn new() -> Self {
        let settings_reader = Arc::new(SettingsReader);

        Self {
            grpc_client: Arc::new(MyLoggerGrpcClient::new(settings_reader)),
        }
    }
}
