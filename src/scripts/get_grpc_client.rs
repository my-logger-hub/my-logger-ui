use std::sync::Arc;

use crate::{app::AppContext, flows::FlowError, grpc_client::MyLoggerGrpcClient};

pub async fn get_grpc_client(
    app: &AppContext,
    env: &str,
) -> Result<Arc<MyLoggerGrpcClient>, FlowError> {
    let settings = app.settings_reader.get_settings().await;

    app.logger_grpc_clients
        .get_or_create(env, &settings)
        .await
        .ok_or_else(|| FlowError::EnvNotFound(env.to_string()))
}
