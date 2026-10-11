use rest_api_shared::ProductScopesHttpModel;

use crate::app::AppContext;

use super::FlowError;

pub async fn get_product_scopes(
    app: &AppContext,
    env: &str,
) -> Result<ProductScopesHttpModel, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, env).await?;

    let response = grpc_client.get_product_scopes(()).await?;

    Ok(response.into())
}
