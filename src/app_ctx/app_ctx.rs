use std::{collections::HashMap, sync::Arc};

use crate::{grpc_client::MyLoggerGrpcClient, settings_model::SettingsModel};
use my_settings_reader::SettingsReader;
use my_ssh::SshSessionsPool;
use rust_extensions::str_utils::StrUtils;
use tokio::sync::Mutex;

use super::{grpc_log_client_settings::GrpcLogSettings, GrpcClient};

pub struct AppContext {
    pub settings_reader: SettingsReader<SettingsModel>,
    pub clients_cache: Mutex<HashMap<String, Arc<GrpcClient>>>,
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

    pub async fn get_client(&self, env: &str) -> Arc<GrpcClient> {
        let mut clients_cache = self.clients_cache.lock().await;

        if let Some(result) = clients_cache.get(env).cloned() {
            return result;
        }

        let settings = self.settings_reader.get_settings().await;
        let connection_settings = settings.get_env_url(env);

        let port_forward_listen_host = build_port_forward_listen_port(env).await;

        let (host, port) = get_host_port(&connection_settings.remote_resource_string);

        let grpc_client = MyLoggerGrpcClient::new(Arc::new(GrpcLogSettings::new(
            port_forward_listen_host.clone(),
        )));

        let mut client = GrpcClient {
            grpc_client,
            ssh_credentials: connection_settings.ssh_credentials.map(Arc::new),
            ssh_port_forward_tunnel: None,
        };

        if let Some(ssh_credentials) = &client.ssh_credentials {
            let ssh_session = self.ssh_sessions_pool.get_or_create(ssh_credentials).await;

            let ssh_session = ssh_session
                .start_port_forward(port_forward_listen_host, host, port)
                .await
                .unwrap();

            client.ssh_port_forward_tunnel = Some(ssh_session);
        }

        let client = Arc::new(client);

        clients_cache.insert(env.to_string(), client.clone());

        client
    }
}

pub async fn build_port_forward_listen_port(env: &str) -> String {
    // return "127.0.0.1:65000".to_string();

    let unix_host = rust_extensions::file_utils::format_path(format!("~/{}.sock", env));

    let _ = tokio::fs::remove_file(unix_host.as_str()).await;

    unix_host.to_string()
}

pub fn get_host_port(src: &str) -> (String, u16) {
    let parts = src.split_2_or_3_lines(":");

    if parts.is_none() {
        panic!("Invalid scheme://host:port format: {}", src);
    }

    let (left, middle, right) = parts.unwrap();

    let (host, port) = if let Some(right) = right {
        (middle, right)
    } else {
        (left, middle)
    };

    let port = port.parse().unwrap();
    if host.starts_with("//") {
        return (host[2..].to_string(), port);
    }
    (host.to_string(), port)
}
