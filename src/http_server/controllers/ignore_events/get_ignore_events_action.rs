use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/ignore-events/v1/list",
    controller: "IgnoreEvents",
    summary: "Get ignore events",
    description: "Returns the ignore events of the env",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "Ignore events", model: "Vec<IgnoreEventApiModel>"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetIgnoreEventsAction {
    app: Arc<AppContext>,
}

impl GetIgnoreEventsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetIgnoreEventsAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_ignore_events(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
