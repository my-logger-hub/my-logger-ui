use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn get_server_info(env: String) -> Result<ServerInfoHttpModel, RequestError> {
    let response = FlUrl::new("/api/server/v1/info")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}
