use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/logs/v1/search",
    controller: "Logs",
    summary: "Search logs",
    description: "Returns the log events of the time range which contain the phrase",
    input_data: "SearchLogsHttpRequest",
    result: [
        {status_code: 200, description: "Log events", model: "Vec<LogApiItem>"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct SearchLogsAction {
    app: Arc<AppContext>,
}

impl SearchLogsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SearchLogsAction,
    input_data: SearchLogsHttpRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::search_logs(&action.app, input_data).await?;
    super::log_events_result(ctx, result)
}
