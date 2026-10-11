use my_http_utils::macros::{MyHttpInput, MyHttpInputObjectStructure, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

use crate::LogApiLevel;

/// Goes both ways: a context of a log event in a response and a filter condition in a request.
#[derive(Serialize, Deserialize, MyHttpInputObjectStructure, Clone, Debug, PartialEq)]
pub struct LogEventContextApiModel {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct LogApiItem {
    pub timestamp: i64,
    pub tenant_id: String,
    pub process_name: String,
    pub message: String,
    pub level: LogApiLevel,
    pub ctx: Vec<LogEventContextApiModel>,
}

/// `from_time` / `to_time` are passed to MyLogger server as they are - encoded the way
/// `ReadLogEventRequest` of MyLogger.proto expects them.
#[derive(MyHttpInput)]
pub struct LoadLogsHttpRequest {
    #[http_body(name = "env", description = "Environment to read the logs from")]
    pub env: String,

    #[http_body(
        name = "level",
        description = "Level of the log events. Absent - any level"
    )]
    pub level: Option<LogApiLevel>,

    #[http_body(name = "fromTime", description = "Beginning of the time range")]
    pub from_time: i64,

    #[http_body(name = "toTime", description = "End of the time range")]
    pub to_time: i64,

    #[http_body(
        name = "ctx",
        description = "Context key-values a log event must have. Empty - no filter"
    )]
    pub ctx: Vec<LogEventContextApiModel>,

    #[http_body(
        name = "productScope",
        description = "Product scope to read the logs of. Absent or empty - the default one"
    )]
    pub product_scope: Option<String>,
}

/// `from_time` / `to_time` are passed to MyLogger server as they are - encoded the way
/// `ScanAndSearchRequest` of MyLogger.proto expects them.
#[derive(MyHttpInput)]
pub struct SearchLogsHttpRequest {
    #[http_body(name = "env", description = "Environment to search the logs in")]
    pub env: String,

    #[http_body(name = "fromTime", description = "Beginning of the time range")]
    pub from_time: i64,

    #[http_body(name = "toTime", description = "End of the time range")]
    pub to_time: i64,

    #[http_body(name = "phrase", description = "Text to search for")]
    pub phrase: String,

    #[http_body(
        name = "productScope",
        description = "Product scope to search the logs in. Absent or empty - the default one"
    )]
    pub product_scope: Option<String>,
}
