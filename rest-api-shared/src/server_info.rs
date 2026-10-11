use my_http_utils::macros::MyHttpObjectStructure;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq, Eq)]
pub struct ServerInfoHttpModel {
    pub version: String,
    pub hours_to_gc: u32,
}
