use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::handle_http_response;

pub async fn search_logs(
    env: String,
    from_time: i64,
    to_time: i64,
    phrase: String,
) -> Result<Vec<LogApiItem>, RequestError> {
    let request = SearchLogsHttpRequest {
        env,
        from_time,
        to_time,
        phrase,
    };

    let response = FlUrl::new("/api/logs/v1/search")
        .execute_request(HttpVerb::Post, request)
        .await;

    handle_http_response(response).await
}

pub async fn load_logs(
    env: String,
    level: Option<LogApiLevel>,
    from_time: i64,
    to_time: i64,
    ctx: Vec<LogEventContextApiModel>,
) -> Result<Vec<LogApiItem>, RequestError> {
    let request = LoadLogsHttpRequest {
        env,
        level,
        from_time,
        to_time,
        ctx,
    };

    let response = FlUrl::new("/api/logs/v1/load")
        .execute_request(HttpVerb::Post, request)
        .await;

    handle_http_response(response).await
}
