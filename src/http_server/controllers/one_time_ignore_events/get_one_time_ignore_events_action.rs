use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/one-time-ignore-events/v1/list",
    controller: "OneTimeIgnoreEvents",
    summary: "Get one time ignore events",
    description: "Returns the one time ignore events of the env",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "One time ignore events", model: "Vec<OneTimeIgnoreHttpModel>"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetOneTimeIgnoreEventsAction {
    app: Arc<AppContext>,
}

impl GetOneTimeIgnoreEventsAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetOneTimeIgnoreEventsAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_one_time_ignore_events(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
