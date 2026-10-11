use rest_api_shared::SaveOneTimeIgnoreEventHttpRequest;

use crate::app::AppContext;

use super::FlowError;

pub async fn save_one_time_ignore_event(
    app: &AppContext,
    request: SaveOneTimeIgnoreEventHttpRequest,
) -> Result<(), FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, &request.env).await?;

    grpc_client
        .set_ignore_single_event(request.item.into())
        .await?;

    Ok(())
}
