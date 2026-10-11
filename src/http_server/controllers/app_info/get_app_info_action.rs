use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/app/v1/info",
    controller: "App",
    summary: "Get app info",
    description: "Returns the version of the app and the envs it is configured with",
    input_data: "GetAppInfoHttpRequest",
    result: [
        {status_code: 200, description: "App info", model: "AppInfoHttpModel"},
    ]
)]
pub struct GetAppInfoAction {
    app: Arc<AppContext>,
}

impl GetAppInfoAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetAppInfoAction,
    input_data: GetAppInfoHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_app_info(&action.app, &input_data.ui_url).await;
    HttpOutput::as_json(result).into_ok_result(true)
}
