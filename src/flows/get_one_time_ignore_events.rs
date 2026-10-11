use rest_api_shared::OneTimeIgnoreHttpModel;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_one_time_ignore_events(
    app: &AppContext,
    env: &str,
) -> Result<Vec<OneTimeIgnoreHttpModel>, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, env).await?;

    let result = grpc_client
        .get_ignore_single_events(())
        .await?
        .into_vec()
        .await?;

    Ok(result)
}
