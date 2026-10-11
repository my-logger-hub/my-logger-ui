use rest_api_shared::{LogApiItem, SearchLogsHttpRequest};

use crate::{app::AppContext, my_logger_grpc::ScanAndSearchRequest};

use super::{FlowError, LOGS_TO_TAKE};

pub async fn search_logs(
    app: &AppContext,
    request: SearchLogsHttpRequest,
) -> Result<Vec<LogApiItem>, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, &request.env).await?;

    let grpc_request = ScanAndSearchRequest {
        ui_url: app.ui_url.get_env_dashboard_url(&request.env).await,
        from_time: request.from_time,
        to_time: request.to_time,
        take: LOGS_TO_TAKE,
        phrase: request.phrase,
    };

    let result = grpc_client
        .scan_and_search(grpc_request)
        .await?
        .into_vec()
        .await?;

    Ok(result)
}
