use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardItem {
    pub hourly: Vec<HourlyStatisticsHttpModel>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyStatisticsHttpModel {
    pub hour_key: i64,
    pub app: String,
    pub info: u32,
    pub warning: u32,
    pub error: u32,
    pub fatal: u32,
    pub debug: u32,
}
