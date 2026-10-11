use my_http_server::HttpFailResult;

use crate::flows::FlowError;

impl From<FlowError> for HttpFailResult {
    fn from(err: FlowError) -> Self {
        match err {
            FlowError::EnvNotFound(env) => {
                HttpFailResult::as_not_found(format!("Env '{}' is not found", env), false)
            }
            FlowError::Grpc(err) => HttpFailResult::as_fatal_error(format!(
                "Request to MyLogger server failed: {:?}",
                err
            )),
        }
    }
}
