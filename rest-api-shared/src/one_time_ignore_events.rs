use std::collections::HashMap;

use my_http_utils::macros::{MyHttpInput, MyHttpInputObjectStructure};
use serde::{Deserialize, Serialize};

use crate::LogApiLevel;

#[derive(Serialize, Deserialize, MyHttpInputObjectStructure, Clone, Debug, PartialEq)]
pub struct OneTimeIgnoreHttpModel {
    pub id: String,
    pub levels: Vec<LogApiLevel>,
    pub message_match: String,
    pub ctx_match: Option<HashMap<String, String>>,
    pub skip_amount: u64,
    pub minutes_to_wait: u64,
}

/// Creates the item or replaces the one with the same id.
#[derive(MyHttpInput)]
pub struct SaveOneTimeIgnoreEventHttpRequest {
    #[http_body(name = "env", description = "Environment the item belongs to")]
    pub env: String,

    #[http_body(name = "item", description = "One time ignore item")]
    pub item: OneTimeIgnoreHttpModel,
}

#[derive(MyHttpInput)]
pub struct DeleteOneTimeIgnoreEventHttpRequest {
    #[http_body(name = "env", description = "Environment the item belongs to")]
    pub env: String,

    #[http_body(name = "id", description = "Id of the item")]
    pub id: String,
}
