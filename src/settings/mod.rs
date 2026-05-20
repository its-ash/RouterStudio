use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    pub openrouter_api_key: String,
    pub github_registry_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            openrouter_api_key: String::new(),
            github_registry_url: "https://raw.githubusercontent.com/its-ash/RouterStudio/main/examples/registry.yaml".to_string(),
        }
    }
}

impl Settings {
    pub fn load(storage: &Storage) -> Self {
        let settings_file = storage.settings_file();

        if let Ok(content) = fs::read_to_string(settings_file) {
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self, storage: &Storage) {
        let settings_file = storage.settings_file();
        if let Ok(content) = serde_json::to_string_pretty(self) {
            fs::write(settings_file, content).ok();
        }
    }
}
