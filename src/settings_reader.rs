use crate::grpc_client::*;

pub struct SettingsReader;

#[async_trait::async_trait]
impl my_grpc_extensions::GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        if name == MyLoggerGrpcClient::get_service_name() {
            return read_env_variable("SERVICE_GRPC_URL");
        }

        panic!("Unknown grpc service name: {}", name)
    }
}

fn read_env_variable(name: &str) -> String {
    match std::env::var(name) {
        Ok(url) => return url,
        Err(_) => panic!("{} is not set", name),
    }
}
