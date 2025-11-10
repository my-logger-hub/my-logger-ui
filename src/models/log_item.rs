use serde::*;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogApiItem {
    pub timestamp: i64,
    pub tenant_id: String,
    pub process_name: String,
    pub message: String,
    pub level: LogApiLevel,
    pub ctx: Vec<LogEventContextApiModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEventContextApiModel {
    pub key: String,
    pub value: String,
}
