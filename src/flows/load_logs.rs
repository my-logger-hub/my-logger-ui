use rest_api_shared::{LoadLogsHttpRequest, LogApiItem};

use crate::{
    app::AppContext,
    my_logger_grpc::{LogLevelGrpcModel, ReadLogEventRequest},
};

use super::{FlowError, LOGS_TO_TAKE};

pub async fn load_logs(
    app: &AppContext,
    request: LoadLogsHttpRequest,
) -> Result<Vec<LogApiItem>, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, &request.env).await?;

    let levels = match request.level {
        Some(level) => vec![LogLevelGrpcModel::from(level) as i32],
        None => vec![],
    };

    let grpc_request = ReadLogEventRequest {
        ui_url: app.ui_url.get_env_dashboard_url(&request.env).await,
        from_time: request.from_time,
        to_time: request.to_time,
        take: LOGS_TO_TAKE,
        skip: 0,
        levels,
        context_keys: request.ctx.into_iter().map(|itm| itm.into()).collect(),
    };

    let result = grpc_client.read(grpc_request).await?.into_vec().await?;

    Ok(result)
}
