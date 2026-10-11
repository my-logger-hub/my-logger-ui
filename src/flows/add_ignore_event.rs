use rest_api_shared::IgnoreEventHttpRequest;

use crate::app::AppContext;

use super::FlowError;

pub async fn add_ignore_event(
    app: &AppContext,
    request: IgnoreEventHttpRequest,
) -> Result<(), FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, &request.env).await?;

    grpc_client.set_ignore_event(request.event.into()).await?;

    Ok(())
}
