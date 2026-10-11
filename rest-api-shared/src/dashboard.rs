use my_http_utils::macros::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct DashboardItem {
    pub hourly: Vec<HourlyStatisticsHttpModel>,
}

/// The counters of an application of an hour. `scope` - the product scope its records went to
#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct HourlyStatisticsHttpModel {
    pub hour_key: i64,
    pub app: String,
    pub scope: String,
    pub info: u32,
    pub warning: u32,
    pub error: u32,
    pub fatal: u32,
    pub debug: u32,
}
