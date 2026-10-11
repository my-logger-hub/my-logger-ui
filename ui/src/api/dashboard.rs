use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn get_dashboard(env: String) -> Result<DashboardItem, RequestError> {
    let response = FlUrl::new("/api/dashboard/v1/hourly-statistics")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}
