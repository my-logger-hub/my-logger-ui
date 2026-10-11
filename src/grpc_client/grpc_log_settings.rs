use my_grpc_extensions::{GrpcClientSettings, GrpcUrl};

use super::MyLoggerGrpcClient;

/// Url of MyLogger server of one env - `ssh:...->http://...` when it is reached over ssh.
pub struct GrpcLogSettings {
    url: String,
}

impl GrpcLogSettings {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait::async_trait]
impl GrpcClientSettings for GrpcLogSettings {
    async fn get_grpc_url(&self, name: &'static str) -> GrpcUrl {
        if name == MyLoggerGrpcClient::get_service_name() {
            return self.url.clone().into();
        }

        panic!("Unknown grpc service name: {}", name);
    }
}
