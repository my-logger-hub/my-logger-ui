use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn get_app_info(ui_url: String) -> Result<AppInfoHttpModel, RequestError> {
    let response = FlUrl::new("/api/app/v1/info")
        .execute_request(HttpVerb::Get, GetAppInfoHttpRequest { ui_url })
        .await;

    handle_http_response(response).await
}
