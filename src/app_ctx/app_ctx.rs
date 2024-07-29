use std::{collections::HashMap, sync::Arc};

use crate::{grpc_client::MyLoggerGrpcClient, settings_model::SettingsModel};
use my_settings_reader::SettingsReader;
use my_ssh::SshSessionsPool;
use tokio::sync::Mutex;

use super::grpc_log_client_settings::GrpcLogSettings;

pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub clients_cache: Mutex<HashMap<String, Arc<MyLoggerGrpcClient>>>,
    pub ssh_sessions_pool: Arc<SshSessionsPool>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            settings_reader: SettingsReader::new("~/.my-logger-ui"),
            clients_cache: Mutex::new(HashMap::default()),
            ssh_sessions_pool: SshSessionsPool::new().into(),
        }
    }

    pub async fn get_client(&self, env: &str) -> Arc<MyLoggerGrpcClient> {
        let mut clients_cache = self.clients_cache.lock().await;

        if let Some(result) = clients_cache.get(env).cloned() {
            return result;
        }

        let settings = self.settings_reader.get_settings().await;
        let over_ssh_connection = settings.get_env_url(env).await;

        let grpc_client = MyLoggerGrpcClient::new(Arc::new(GrpcLogSettings::new(
            over_ssh_connection.remote_resource_string,
        )));

        if let Some(value) = over_ssh_connection.ssh_credentials {
            grpc_client.set_ssh_credentials(Arc::new(value)).await;
            grpc_client
                .set_ssh_sessions_pool(self.ssh_sessions_pool.clone())
                .await;
        };

        let grpc_client = Arc::new(grpc_client);

        clients_cache.insert(env.to_string(), grpc_client.clone());

        grpc_client
    }
}
