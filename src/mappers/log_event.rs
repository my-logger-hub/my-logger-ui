use rest_api_shared::LogApiItem;

use crate::my_logger_grpc::LogEventGrpcModel;

impl From<LogEventGrpcModel> for LogApiItem {
    fn from(value: LogEventGrpcModel) -> Self {
        let level = value.level().into();

        Self {
            timestamp: value.timestamp,
            tenant_id: value.tenant_id,
            process_name: value.process_name,
            message: value.message,
            level,
            ctx: value.ctx.into_iter().map(|itm| itm.into()).collect(),
        }
    }
}
