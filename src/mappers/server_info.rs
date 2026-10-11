use rest_api_shared::ServerInfoHttpModel;

use crate::my_logger_grpc::ServerInfoGrpcResponse;

impl From<ServerInfoGrpcResponse> for ServerInfoHttpModel {
    fn from(value: ServerInfoGrpcResponse) -> Self {
        Self {
            version: value.version,
            hours_to_gc: value.hours_to_gc,
        }
    }
}
