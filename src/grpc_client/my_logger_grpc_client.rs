use my_grpc_extensions::client::*;

#[generate_grpc_client(
    proto_file: "./proto/MyLogger.proto",
    crate_ns: "crate::my_logger_grpc",
    retries: 3,
    request_timeout_sec: 1,
    ping_timeout_sec: 1,
    ping_interval_sec: 3,
)]
pub struct MyLoggerGrpcClient;
