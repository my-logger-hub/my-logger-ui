use crate::models::*;
use dioxus::prelude::*;

#[server]
pub async fn get_ignore_events(env: String) -> Result<Vec<IgnoreEventApiModel>, ServerFnError> {
    let result: Vec<IgnoreEventApiModel> = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .get_ignore_events(())
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(result)
}

#[server]
pub async fn add_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let level: LogLevelGrpcModel = event.level.as_ref().into();

    crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .set_ignore_event(IgnoreEventGrpcModel {
            level: level as i32,
            application: event.application,
            marker: event.marker,
            expiration: Default::default(),
        })
        .await
        .unwrap();

    Ok(())
}

#[server]
pub async fn delete_ignore_event(
    env: String,
    event: IgnoreEventApiModel,
) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let level: LogLevelGrpcModel = event.level.as_ref().into();

    crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .delete_ignore_event(DeleteIgnoreEventGrpcRequest {
            level: level as i32,
            application: event.application,
            marker: event.marker,
        })
        .await
        .unwrap();

    Ok(())
}

#[cfg(feature = "server")]
impl From<crate::server::my_logger_grpc::IgnoreEventGrpcModel> for IgnoreEventApiModel {
    fn from(value: crate::server::my_logger_grpc::IgnoreEventGrpcModel) -> Self {
        IgnoreEventApiModel {
            level: value.level().into(),
            application: value.application,
            marker: value.marker,
        }
    }
}
