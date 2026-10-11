use rest_api_shared::DeleteOneTimeIgnoreEventHttpRequest;

use crate::{app::AppContext, my_logger_grpc::DeleteIgnoreSingleEventGrpcRequest};

use super::FlowError;

pub async fn delete_one_time_ignore_event(
    app: &AppContext,
    request: DeleteOneTimeIgnoreEventHttpRequest,
) -> Result<(), FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, &request.env).await?;

    grpc_client
        .delete_ignore_single_event(DeleteIgnoreSingleEventGrpcRequest { id: request.id })
        .await?;

    Ok(())
}
