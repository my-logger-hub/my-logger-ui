use std::sync::Arc;

use my_http_server::macros::*;
use my_http_server::*;
use rest_api_shared::*;

use crate::app::AppContext;

#[http_route(
    method: "GET",
    route: "/api/product-scopes/v1/list",
    controller: "ProductScopes",
    summary: "Get product scopes",
    description: "Returns the product scopes MyLogger server of the env keeps the logs in",
    input_data: "EnvHttpRequest",
    result: [
        {status_code: 200, description: "Product scopes", model: "ProductScopesHttpModel"},
        {status_code: 404, description: "Env is not found"},
    ]
)]
pub struct GetProductScopesAction {
    app: Arc<AppContext>,
}

impl GetProductScopesAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &GetProductScopesAction,
    input_data: EnvHttpRequest,
    _ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    let result = crate::flows::get_product_scopes(&action.app, &input_data.env).await?;
    HttpOutput::as_json(result).into_ok_result(true)
}
