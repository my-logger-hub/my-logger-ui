use rest_api_shared::ServerInfoHttpModel;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_server_info(
    app: &AppContext,
    env: &str,
) -> Result<ServerInfoHttpModel, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, env).await?;

    // An unreachable server is not an error here: the UI shows it as not connected
    let result = match grpc_client.get_server_info(()).await {
        Ok(server_info) => server_info.into(),
        Err(_) => ServerInfoHttpModel {
            version: String::new(),
            hours_to_gc: 0,
        },
    };

    Ok(result)
}
