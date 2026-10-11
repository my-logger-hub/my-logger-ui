use my_http_utils::macros::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct GetAppInfoHttpRequest {
    #[http_query(
        name = "uiUrl",
        description = "Origin the UI is opened at. An https one is used to build links to the UI"
    )]
    pub ui_url: String,
}

#[derive(Serialize, Deserialize, MyHttpObjectStructure, Clone, Debug, PartialEq)]
pub struct AppInfoHttpModel {
    pub version: String,
    pub envs: Vec<String>,
}
