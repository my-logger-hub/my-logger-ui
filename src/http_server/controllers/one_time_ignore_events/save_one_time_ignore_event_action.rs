use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/one-time-ignore-events/v1/save",
    controller: "OneTimeIgnoreEvents",
    summary: "Save one time ignore event",
    description: "Creates the one time ignore event or replaces the one with the same id",
    input_data: "SaveOneTimeIgnoreEventHttpRequest",
    result: [
        {status_code: 204, description: "One time ignore event is saved"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct SaveOneTimeIgnoreEventAction {
    app: Arc<AppContext>,
}

impl SaveOneTimeIgnoreEventAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &SaveOneTimeIgnoreEventAction,
    input_data: SaveOneTimeIgnoreEventHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::save_one_time_ignore_event(&action.app, input_data).await?;
    HttpOutput::Empty.into_ok_result(true)
}
