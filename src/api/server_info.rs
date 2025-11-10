use crate::models::*;
use dioxus::prelude::*;

#[get("/api/server_info?env")]
pub async fn get_server_info(env: String) -> Result<ServerInfoHttpModel, ServerFnError> {
    match crate::server::APP_CTX
        .get_client(env.as_str())
        .await
        .get_server_info(())
        .await
    {
        Ok(result) => Ok(ServerInfoHttpModel {
            version: result.version,
            hours_to_gc: result.hours_to_gc,
        }),
        Err(_) => Ok(ServerInfoHttpModel {
            version: String::new(),
            hours_to_gc: 0,
        }),
    }
}

#[get("/api/envs?ui_url")]
pub async fn get_envs(ui_url: String) -> Result<Vec<String>, ServerFnError> {
    let mut ui_url = ui_url;
    if ui_url.starts_with("https") {
        if ui_url.ends_with("/") {
            ui_url.push_str("dashboard/");
        } else {
            ui_url.push_str("/dashboard/");
        }
        crate::server::APP_CTX.set_ui_url(ui_url).await;
    }

    let result = crate::server::APP_CTX
        .settings_reader
        .get_settings()
        .await
        .get_envs();

    Ok(result)
}
