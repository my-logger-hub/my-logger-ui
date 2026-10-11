use rest_api_shared::AppInfoHttpModel;

use crate::app::{AppContext, APP_VERSION};

pub async fn get_app_info(app: &AppContext, ui_url: &str) -> AppInfoHttpModel {
    // Links to the UI are given to MyLogger server only when the UI is opened over https
    if ui_url.starts_with("https") {
        let dashboard_url = if ui_url.ends_with('/') {
            format!("{}dashboard/", ui_url)
        } else {
            format!("{}/dashboard/", ui_url)
        };

        app.ui_url.set_dashboard_url(dashboard_url).await;
    }

    let settings = app.settings_reader.get_settings().await;

    AppInfoHttpModel {
        version: APP_VERSION.to_string(),
        envs: settings.get_envs(),
    }
}
