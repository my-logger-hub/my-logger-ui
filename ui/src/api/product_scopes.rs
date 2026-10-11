use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn get(env: String) -> Result<ProductScopesHttpModel, RequestError> {
    let response = FlUrl::new("/api/product-scopes/v1/list")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}
