use crate::models::*;
use dioxus::prelude::*;

#[post("/api/logs/search")]
pub async fn search_logs(
    env: String,
    from_time: i64,
    to_time: i64,
    phrase: String,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let ui_url = crate::server::APP_CTX.get_ui_url(&env).await;
    let result = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .scan_and_search(ScanAndSearchRequest {
            ui_url,
            from_time: from_time,
            to_time: to_time,
            take: 200,
            phrase,
        })
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(result)
}

#[post("/api/logs/load")]
pub async fn load_logs(
    env: String,
    level: Option<LogApiLevel>,
    from_time: i64,
    to_time: i64,
    ctx: Option<Vec<LogEventContextApiModel>>,
) -> Result<Vec<LogApiItem>, ServerFnError> {
    use crate::server::my_logger_grpc::*;
    println!("Load logs '{}'-'{}'", from_time, to_time);

    let levels = if let Some(level) = level {
        match level {
            LogApiLevel::Info => vec![LogLevelGrpcModel::Info as i32],
            LogApiLevel::Warning => vec![LogLevelGrpcModel::Warning as i32],
            LogApiLevel::Error => vec![LogLevelGrpcModel::Error as i32],
            LogApiLevel::FatalError => vec![LogLevelGrpcModel::Fatal as i32],
            LogApiLevel::Debug => vec![LogLevelGrpcModel::Debug as i32],
        }
    } else {
        vec![]
    };

    let ctx = ctx.unwrap_or_default();

    let result = crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .read(ReadLogEventRequest {
            ui_url: crate::server::APP_CTX.get_ui_url(&env).await,
            from_time: from_time,
            to_time: to_time,
            levels,
            context_keys: ctx
                .into_iter()
                .map(|itm| LogEventContext {
                    key: itm.key,
                    value: itm.value,
                })
                .collect(),
            take: 200,
            skip: 0,
        })
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(result)
}

#[cfg(feature = "server")]
impl From<crate::server::my_logger_grpc::LogEventGrpcModel> for LogApiItem {
    fn from(value: crate::server::my_logger_grpc::LogEventGrpcModel) -> Self {
        let level: LogApiLevel = (&value.level()).into();

        LogApiItem {
            timestamp: value.timestamp,
            tenant_id: value.tenant_id,
            process_name: value.process_name,
            message: value.message,
            level,
            ctx: value
                .ctx
                .into_iter()
                .map(|itm| LogEventContextApiModel {
                    key: itm.key,
                    value: itm.value,
                })
                .collect(),
        }
    }
}
