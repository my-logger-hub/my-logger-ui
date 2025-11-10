use serde::*;

use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IgnoreEventApiModel {
    pub level: LogApiLevel,
    pub application: String,
    pub marker: String,
}
