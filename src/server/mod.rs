mod app_ctx;
mod grpc_client;
mod settings;

pub use crate::server::app_ctx::*;

lazy_static::lazy_static! {
    pub static ref APP_CTX: AppContext = {
       AppContext::new()
    };
}

pub mod my_logger_grpc {
    tonic::include_proto!("my_logger");
}
