use std::sync::Arc;

use crate::grpc_client::MyLoggerGrpcClient;

pub struct GrpcClient {
    pub grpc_client: MyLoggerGrpcClient,
    pub ssh_credentials: Option<Arc<my_ssh::SshCredentials>>,
    pub ssh_port_forward_tunnel: Option<Arc<my_ssh::SshPortForwardTunnel>>,
}
