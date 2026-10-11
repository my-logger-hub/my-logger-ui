use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::{handle_http_empty, handle_http_response};

pub async fn get_ignore_events(env: String) -> Result<Vec<IgnoreEventApiModel>, RequestError> {
    let response = FlUrl::new("/api/ignore-events/v1/list")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}

pub async fn add_ignore_event(env: String, event: IgnoreEventApiModel) -> Result<(), RequestError> {
    let response = FlUrl::new("/api/ignore-events/v1/add")
        .execute_request(HttpVerb::Post, IgnoreEventHttpRequest { env, event })
        .await;

    handle_http_empty(response).await
}

pub async fn delete_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), RequestError> {
    let response = FlUrl::new("/api/ignore-events/v1/delete")
        .execute_request(HttpVerb::Post, IgnoreEventHttpRequest { env, event })
        .await;

    handle_http_empty(response).await
}
