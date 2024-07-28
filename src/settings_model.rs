use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SettingsModel {
    pub envs: BTreeMap<String, String>,
}

impl SettingsModel {
    pub fn get_env_url(&self, env: &str) -> my_ssh::OverSshConnectionSettings {
        if let Some(result) = self.envs.get(env) {
            return my_ssh::OverSshConnectionSettings::parse(result);
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
        };

        settings.envs.insert(
            "dev".to_string(),
            "ssh:root@10.0.0.0:22->grpc://localhost:50051".to_string(),
        );

        println!("{}", serde_yaml::to_string(&settings).unwrap());
    }
}
