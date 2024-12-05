use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, String>,
    pub ssh_private_keys: Option<HashMap<String, my_ssh::SshPrivateKeySettingsModel>>,
}

impl SettingsModel {
    pub fn get_envs(&self) -> Vec<String> {
        self.envs.keys().cloned().collect()
    }
    pub async fn get_env_url(&self, env: &str) -> String {
        if let Some(result) = self.envs.get(env) {
            return result.to_string();
        }

        panic!("Can not get settings for env: '{}'", env);
    }
}

#[async_trait::async_trait]
impl my_ssh::SshPrivateKeyResolver for SettingsModel {
    async fn resolve_ssh_private_key(&self, ssh_line: &str) -> Option<my_ssh::SshPrivateKey> {
        let private_keys = self.ssh_private_keys.as_ref()?;

        if let Some(ssh_credentials) = private_keys.get(ssh_line) {
            return my_ssh::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        if let Some(ssh_credentials) = private_keys.get("*") {
            return my_ssh::SshPrivateKey {
                content: ssh_credentials.load_cert().await,
                pass_phrase: ssh_credentials.cert_pass_phrase.clone(),
            }
            .into();
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::SettingsModel;

    #[test]
    fn test() {
        let mut settings = SettingsModel {
            envs: std::collections::BTreeMap::new(),
            ssh_private_keys: None,
        };

        settings.envs.insert(
            "dev".to_string(),
            "ssh:root@10.0.0.0:22->grpc://localhost:50051".to_string(),
        );

        println!("{}", serde_yaml::to_string(&settings).unwrap());
    }
}
