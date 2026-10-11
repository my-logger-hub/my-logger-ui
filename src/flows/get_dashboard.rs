use rest_api_shared::DashboardItem;

use crate::{app::AppContext, my_logger_grpc::GetHourlyStatisticsRequest};

use super::{FlowError, DASHBOARD_HOURS_AMOUNT};

pub async fn get_dashboard(app: &AppContext, env: &str) -> Result<DashboardItem, FlowError> {
    let grpc_client = crate::scripts::get_grpc_client(app, env).await?;

    let hourly = grpc_client
        .get_hourly_statistics(GetHourlyStatisticsRequest {
            amount_of_hours: DASHBOARD_HOURS_AMOUNT,
        })
        .await?
        .into_vec()
        .await?;

    Ok(DashboardItem { hourly })
}
