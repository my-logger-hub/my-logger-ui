use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "POST",
    route: "/api/one-time-ignore-events/v1/delete",
    controller: "OneTimeIgnoreEvents",
    summary: "Delete one time ignore event",
    description: "Deletes the one time ignore event from the env",
    input_data: "DeleteOneTimeIgnoreEventHttpRequest",
    result: [
        {status_code: 204, description: "One time ignore event is deleted"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct DeleteOneTimeIgnoreEventAction {
    app: Arc<AppContext>,
}

impl DeleteOneTimeIgnoreEventAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &DeleteOneTimeIgnoreEventAction,
    input_data: DeleteOneTimeIgnoreEventHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    crate::flows::delete_one_time_ignore_event(&action.app, input_data).await?;
    HttpOutput::Empty.into_ok_result(true)
}
