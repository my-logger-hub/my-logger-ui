use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn get(env: String) -> Result<Vec<String>, RequestError> {
    let response = FlUrl::new("/api/insights/v1/keys")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}
