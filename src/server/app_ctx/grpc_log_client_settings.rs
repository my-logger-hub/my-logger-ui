use my_grpc_extensions::*;

use crate::server::grpc_client::*;

pub struct GrpcLogSettings(String);

impl GrpcLogSettings {
    pub fn new(host: String) -> Self {
        Self(host)
    }
}

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for GrpcLogSettings {
    async fn get_grpc_url(&self, name: &'static str) -> GrpcUrl {
        if name == MyLoggerGrpcClient::get_service_name() {
            return self.0.to_string().into();
        }

        panic!("Unknown service name: {}", name);
    }
}
