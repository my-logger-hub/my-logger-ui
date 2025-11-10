use crate::models::*;
use dioxus::prelude::*;

#[get("/api/dashboard?env")]
pub async fn get_dashboard(env: String) -> Result<DashboardItem, ServerFnError> {
    use crate::server::my_logger_grpc::*;

    let client = crate::server::APP_CTX.get_client(env.as_str()).await;

    let items: Vec<HourlyStatisticsHttpModel> = client
        .get_hourly_statistics(GetHourlyStatisticsRequest { amount_of_hours: 8 })
        .await
        .unwrap()
        .into_vec()
        .await
        .unwrap();

    Ok(DashboardItem { hourly: items })
}

#[cfg(feature = "server")]
impl From<crate::server::my_logger_grpc::HourlyStatisticsGrpcModel> for HourlyStatisticsHttpModel {
    fn from(value: crate::server::my_logger_grpc::HourlyStatisticsGrpcModel) -> Self {
        HourlyStatisticsHttpModel {
            hour_key: value.hour_key as i64,
            app: value.app,
            info: value.info_count,
            warning: value.warning_count,
            error: value.error_count,
            fatal: value.fatal_count,
            debug: value.debug_count,
        }
    }
}
