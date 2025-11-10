use std::collections::HashMap;

use serde::*;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OneTimeIgnoreHttpModel {
    pub id: String,
    pub levels: Vec<LogApiLevel>,
    pub message_match: String,
    pub ctx_match: Option<HashMap<String, String>>,
    pub skip_amount: u64,
    pub minutes_to_wait: u64,
}

impl Default for OneTimeIgnoreHttpModel {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            levels: vec![],
            message_match: "".to_string(),
            ctx_match: None,
            skip_amount: 0,
            minutes_to_wait: 0,
        }
    }
}
