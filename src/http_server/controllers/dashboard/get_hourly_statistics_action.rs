use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/dashboard/v1/hourly-statistics",
    controller: "Dashboard",
    summary: "Get hourly statistics",
    description: "Returns the amounts of log events per application and level for the last hours",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "Hourly statistics", model: "DashboardItem"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetHourlyStatisticsAction {
    app: Arc<AppContext>,
}

impl GetHourlyStatisticsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetHourlyStatisticsAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_dashboard(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
