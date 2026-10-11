use crate::app::AppContext;

use super::FlowError;

pub async fn get_insights_keys(app: &AppContext, env: &str) -> Result<Vec<String>, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, env).await?;

    // Insights are a hint for the search line: without them the UI just has nothing to suggest
    let keys = match grpc_client.get_insights_keys(()).await {
        Ok(response) => response.keys,
        Err(_) => vec![],
    };

    Ok(keys)
}
