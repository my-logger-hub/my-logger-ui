use my_http_utils::macros::{MyHttpInput, MyHttpInputObjectStructure};
use serde::{Deserialize, Serialize};

use crate::LogApiLevel;

#[derive(Serialize, Deserialize, MyHttpInputObjectStructure, Clone, Debug, PartialEq)]
pub struct IgnoreEventApiModel {
    pub level: LogApiLevel,
    pub application: String,
    pub marker: String,
}

/// Input of both adding and deleting an ignore event.
#[derive(MyHttpInput)]
pub struct IgnoreEventHttpRequest {
    #[http_body(name = "env", description = "Environment the ignore event belongs to")]
    pub env: String,

    #[http_body(name = "event", description = "Ignore event")]
    pub event: IgnoreEventApiModel,
}
