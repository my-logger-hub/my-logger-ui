use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{grpc_client::MyLoggerGrpcClient, settings_reader::SettingsReader};

pub struct AppContextInner {
    my_logger_grpc_client: Arc<MyLoggerGrpcClient>,
}

pub struct AppContext {
    inner: RwLock<Option<AppContextInner>>,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            inner: RwLock::new(None),
        }
    }
    pub async fn apply_settings(&self, settings_reader: Arc<SettingsReader>) {
        let mut write_access = self.inner.write().await;

        *write_access = Some(AppContextInner {
            my_logger_grpc_client: Arc::new(MyLoggerGrpcClient::new(settings_reader)),
        })
    }

    pub async fn get_my_logger_grpc_client(&self) -> Arc<MyLoggerGrpcClient> {
        let read_access = self.inner.read().await;
        read_access.as_ref().unwrap().my_logger_grpc_client.clone()
    }
}
