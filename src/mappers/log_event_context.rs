use rest_api_shared::LogEventContextApiModel;

use crate::my_logger_grpc::LogEventContext;

impl From<LogEventContext> for LogEventContextApiModel {
    fn from(value: LogEventContext) -> Self {
        Self {
            key: value.key,
            value: value.value,
        }
    }
}

impl From<LogEventContextApiModel> for LogEventContext {
    fn from(value: LogEventContextApiModel) -> Self {
        Self {
            key: value.key,
            value: value.value,
        }
    }
}
