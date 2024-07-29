use std::collections::{BTreeMap, HashMap};

use my_ssh::SshCredentialsSettingsModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, String>,
    pub ssh_credentials: Option<HashMap<String, SshCredentialsSettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
    pub async fn get_env_url(&self, env: &str) -> my_ssh::OverSshConnectionSettings {
        if let Some(result) = self.envs.get(env) {
            return my_ssh::OverSshConnectionSettings::parse(result, self.ssh_credentials.as_ref())
                .await;
        }

        panic!("Can not get settings for env: '{}'", env);
    }
}

#[cfg(test)]
mod test {
    use super::SettingsModel;

    #[test]
    fn test() {
        let mut settings = SettingsModel {
            envs: std::collections::BTreeMap::new(),
            ssh_credentials: None,
        };

        settings.envs.insert(
            "dev".to_string(),
            "ssh:root@10.0.0.0:22->grpc://localhost:50051".to_string(),
        );

        println!("{}", serde_yaml::to_string(&settings).unwrap());
    }
}
