use serde::{Deserialize, Serialize};

use crate::grpc_client::*;

#[derive(my_settings_reader::SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "MyLoggerGrpcUrl")]
    pub my_logger_grpc_url: String,
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == MyLoggerGrpcClient::get_service_name() {
            let read_access = self.settings.read().await;
            return read_access.my_logger_grpc_url.clone();
        }

        panic!("Unknown grpc service name: {}", name)
    }
}
