use tokio::sync::Mutex;

/// Dashboard url of the UI as the browser opened it. MyLogger server gets it with each logs
/// request to build links back to the UI.
pub struct UiUrl {
    dashboard_url: Mutex<String>,
}

impl UiUrl {
    pub fn new() -> Self {
        Self {
            dashboard_url: Mutex::new(String::new()),
        }
    }

    pub async fn set_dashboard_url(&self, dashboard_url: String) {
        let mut current = self.dashboard_url.lock().await;

        if *current != dashboard_url {
            println!("UI URL is set to {}", dashboard_url);
            *current = dashboard_url;
        }
    }

    /// Until the UI is opened over https it is just the env name.
    pub async fn get_env_dashboard_url(&self, env: &str) -> String {
        let current = self.dashboard_url.lock().await;
        format!("{}{}", current.as_str(), env)
    }
}
