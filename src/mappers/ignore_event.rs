use rest_api_shared::IgnoreEventApiModel;

use crate::my_logger_grpc::{
    DeleteIgnoreEventGrpcRequest, IgnoreEventGrpcModel, LogLevelGrpcModel,
};

impl From<IgnoreEventGrpcModel> for IgnoreEventApiModel {
    fn from(value: IgnoreEventGrpcModel) -> Self {
        let level = value.level().into();

        Self {
            level,
            application: value.application,
            marker: value.marker,
        }
    }
}

impl From<IgnoreEventApiModel> for IgnoreEventGrpcModel {
    fn from(value: IgnoreEventApiModel) -> Self {
        Self {
            level: LogLevelGrpcModel::from(value.level) as i32,
            application: value.application,
            marker: value.marker,
            expiration: None,
        }
    }
}

impl From<IgnoreEventApiModel> for DeleteIgnoreEventGrpcRequest {
    fn from(value: IgnoreEventApiModel) -> Self {
        Self {
            level: LogLevelGrpcModel::from(value.level) as i32,
            application: value.application,
            marker: value.marker,
        }
    }
}
