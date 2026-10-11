use rest_api_shared::HourlyStatisticsHttpModel;

use crate::my_logger_grpc::HourlyStatisticsGrpcModel;

impl From<HourlyStatisticsGrpcModel> for HourlyStatisticsHttpModel {
    fn from(value: HourlyStatisticsGrpcModel) -> Self {
        Self {
            hour_key: value.hour_key as i64,
            app: value.app,
            scope: value.scope,
            info: value.info_count,
            warning: value.warning_count,
            error: value.error_count,
            fatal: value.fatal_count,
            debug: value.debug_count,
        }
    }
}
