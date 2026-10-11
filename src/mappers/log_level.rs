use rest_api_shared::LogApiLevel;

use crate::my_logger_grpc::LogLevelGrpcModel;

impl From<LogLevelGrpcModel> for LogApiLevel {
    fn from(value: LogLevelGrpcModel) -> Self {
        match value {
            LogLevelGrpcModel::Info => Self::Info,
            LogLevelGrpcModel::Warning => Self::Warning,
            LogLevelGrpcModel::Error => Self::Error,
            LogLevelGrpcModel::Fatal => Self::FatalError,
            LogLevelGrpcModel::Debug => Self::Debug,
        }
    }
}

impl From<LogApiLevel> for LogLevelGrpcModel {
    fn from(value: LogApiLevel) -> Self {
        match value {
            LogApiLevel::Info => Self::Info,
            LogApiLevel::Warning => Self::Warning,
            LogApiLevel::Error => Self::Error,
            LogApiLevel::FatalError => Self::Fatal,
            LogApiLevel::Debug => Self::Debug,
        }
    }
}
