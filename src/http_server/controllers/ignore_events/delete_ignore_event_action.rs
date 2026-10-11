use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/ignore-events/v1/delete",
    controller: "IgnoreEvents",
    summary: "Delete ignore event",
    description: "Deletes the ignore event from the env",
    input_data: "IgnoreEventHttpRequest",
    result: [
        {status_code: 204, description: "Ignore event is deleted"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct DeleteIgnoreEventAction {
    app: Arc<AppContext>,
}

impl DeleteIgnoreEventAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteIgnoreEventAction,
    input_data: IgnoreEventHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::delete_ignore_event(&action.app, input_data).await?;
    HttpOutput::Empty.into_ok_result(true)
}
