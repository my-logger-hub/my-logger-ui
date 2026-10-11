use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/logs/v1/load",
    controller: "Logs",
    summary: "Load logs",
    description: "Returns the log events of the time range filtered by level and context",
    input_data: "LoadLogsHttpRequest",
    result: [
        {status_code: 200, description: "Log events", model: "Vec<LogApiItem>"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct LoadLogsAction {
    app: Arc<AppContext>,
}

impl LoadLogsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &LoadLogsAction,
    input_data: LoadLogsHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::load_logs(&action.app, input_data).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
