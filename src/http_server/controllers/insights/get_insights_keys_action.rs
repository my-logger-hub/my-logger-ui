use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/insights/v1/keys",
    controller: "Insights",
    summary: "Get insights keys",
    description: "Returns the context keys MyLogger server of the env has insights for",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "Insights keys", model: "Vec<String>"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetInsightsKeysAction {
    app: Arc<AppContext>,
}

impl GetInsightsKeysAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetInsightsKeysAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_insights_keys(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
