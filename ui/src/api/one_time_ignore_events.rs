use flurl::{FlUrl, HttpVerb};

use crate::models::*;

use super::{handle_http_empty, handle_http_response};

pub async fn get(env: String) -> Result<Vec<OneTimeIgnoreHttpModel>, RequestError> {
    let response = FlUrl::new("/api/one-time-ignore-events/v1/list")
        .execute_request(HttpVerb::Get, EnvHttpRequest { env })
        .await;

    handle_http_response(response).await
}

pub async fn save_one_time_ignore_event(
    env: String,
    item: OneTimeIgnoreHttpModel,
) -> Result<(), RequestError> {
    let response = FlUrl::new("/api/one-time-ignore-events/v1/save")
        .execute_request(
            HttpVerb::Post,
            SaveOneTimeIgnoreEventHttpRequest { env, item },
        )
        .await;

    handle_http_empty(response).await
}

pub async fn delete_one_time_ignore_event(env: String, id: String) -> Result<(), RequestError> {
    let response = FlUrl::new("/api/one-time-ignore-events/v1/delete")
        .execute_request(
            HttpVerb::Post,
            DeleteOneTimeIgnoreEventHttpRequest { env, id },
        )
        .await;

    handle_http_empty(response).await
}
