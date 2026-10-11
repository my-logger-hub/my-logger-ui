use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/server/v1/info",
    controller: "Server",
    summary: "Get MyLogger server info",
    description: "Returns the version and the GC settings of MyLogger server of the env. Empty version - the server is not reachable",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "Server info", model: "ServerInfoHttpModel"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetServerInfoAction {
    app: Arc<AppContext>,
}

impl GetServerInfoAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetServerInfoAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_server_info(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
