use crate::my_logger_grpc::*;
use crate::LogApiLevel;

impl Into<LogApiLevel> for &'_ LogLevelGrpcModel {
    fn into(self) -> LogApiLevel {
        match self {
            LogLevelGrpcModel::Info => LogApiLevel::Info,
            LogLevelGrpcModel::Warning => LogApiLevel::Warning,
            LogLevelGrpcModel::Error => LogApiLevel::Error,
            LogLevelGrpcModel::Fatal => LogApiLevel::FatalError,
            LogLevelGrpcModel::Debug => LogApiLevel::Debug,
        }
    }
}

impl Into<LogLevelGrpcModel> for &'_ LogApiLevel {
    fn into(self) -> LogLevelGrpcModel {
        match self {
            LogApiLevel::Info => LogLevelGrpcModel::Info,
            LogApiLevel::Warning => LogLevelGrpcModel::Warning,
            LogApiLevel::Error => LogLevelGrpcModel::Error,
            LogApiLevel::FatalError => LogLevelGrpcModel::Fatal,
            LogApiLevel::Debug => LogLevelGrpcModel::Debug,
        }
    }
}
