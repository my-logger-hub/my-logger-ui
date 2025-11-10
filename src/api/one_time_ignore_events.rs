use crate::models::*;
use dioxus::prelude::*;

#[get("/api/one_time_ignore_events?env")]
pub async fn get(env: String) -> Result<Vec<OneTimeIgnoreHttpModel>, ServerFnError> {
    let response = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .get_ignore_single_events(())
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(response)
}
#[post("/api/one_time_ignore_events/save")]
pub async fn save_one_time_ignore_event(
    env: String,
    itm: OneTimeIgnoreHttpModel,
) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;

    crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .set_ignore_single_event(IgnoreSingleEventGrpcModel {
            id: itm.id,
            levels: itm
                .levels
                .iter()
                .map(|itm| {
                    let log_level: LogLevelGrpcModel = itm.into();
                    let result: i32 = log_level.into();
                    result
                })
                .collect(),
            message_match: itm.message_match,
            context_match: if let Some(ctx_match) = itm.ctx_match.as_ref() {
                ctx_match
                    .iter()
                    .map(|(k, v)| LogEventContext {
                        key: k.to_string(),
                        value: v.to_string(),
                    })
                    .collect()
            } else {
                vec![]
            },
            skip_amount: itm.skip_amount,
            minutes_to_wait: itm.minutes_to_wait,
        })
        .await
        .unwrap();

    Ok(())
}

#[post("/api/one_time_ignore_events/delete")]
pub async fn delete_one_time_ignore_event(env: String, id: String) -> Result<(), ServerFnError> {
    use crate::server::my_logger_grpc::*;
    crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .delete_ignore_single_event(DeleteIgnoreSingleEventGrpcRequest { id })
        .await
        .unwrap();

    Ok(())
}

#[cfg(feature = "server")]
impl From<crate::server::my_logger_grpc::IgnoreSingleEventGrpcModel> for OneTimeIgnoreHttpModel {
    fn from(value: crate::server::my_logger_grpc::IgnoreSingleEventGrpcModel) -> Self {
        let levels = value.levels().map(|itm| itm.into()).collect();

        let ctx_match = if value.context_match.len() > 0 {
            Some(
                value
                    .context_match
                    .iter()
                    .map(|itm| (itm.key.to_string(), itm.value.to_string()))
                    .collect(),
            )
        } else {
            None
        };

        OneTimeIgnoreHttpModel {
            id: value.id,
            levels,
            message_match: value.message_match,
            ctx_match,
            skip_amount: value.skip_amount,
            minutes_to_wait: value.minutes_to_wait,
        }
    }
}
