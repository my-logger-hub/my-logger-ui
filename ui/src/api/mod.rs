pub mod app_info;
pub mod dashboard;
pub mod ignore_events;
pub mod insights;
pub mod logs;
pub mod one_time_ignore_events;
pub mod product_scopes;
pub mod server_info;

use flurl::{FlUrlError, FlUrlResponse};
use serde::de::DeserializeOwned;

use crate::models::RequestError;

/// Only a guard - the biggest response is one page of log events.
const MAX_RESPONSE_BODY_SIZE: usize = 64 * 1024 * 1024;

fn is_success(status: u16) -> bool {
    (200..300).contains(&status)
}

async fn read_error_body(response: &mut FlUrlResponse) -> RequestError {
    let status_code = response.get_status_code();

    let body = match response.get_body() {
        Ok(body) => body.into_string(MAX_RESPONSE_BODY_SIZE).await,
        Err(err) => Err(err),
    };

    let message = match body {
        Ok(body) => format!("[{}] {}", status_code, body),
        Err(err) => format!("[{}] {}", status_code, err),
    };

    RequestError { message }
}

/// 2xx -> the body deserialized into `T`; any other status -> `Err` with the response body.
async fn handle_http_response<T: DeserializeOwned>(
    response: Result<FlUrlResponse, FlUrlError>,
) -> Result<T, RequestError> {
    let mut response = response?;

    if is_success(response.get_status_code()) {
        let result = response
            .get_body()?
            .into_json(MAX_RESPONSE_BODY_SIZE)
            .await?;

        return Ok(result);
    }

    Err(read_error_body(&mut response).await)
}

/// Endpoints with no response body: 2xx -> `Ok(())`, otherwise `Err` with the response body.
async fn handle_http_empty(
    response: Result<FlUrlResponse, FlUrlError>,
) -> Result<(), RequestError> {
    let mut response = response?;

    if is_success(response.get_status_code()) {
        return Ok(());
    }

    Err(read_error_body(&mut response).await)
}
