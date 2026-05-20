use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Storage {
    base_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let base_dir = dirs::home_dir()
            .expect("Failed to get home directory")
            .join(".openagent");

        fs::create_dir_all(&base_dir).ok();
        fs::create_dir_all(base_dir.join("agents")).ok();
        fs::create_dir_all(base_dir.join("cache")).ok();

        Self { base_dir }
    }

    pub fn base_dir(&self) -> &PathBuf {
        &self.base_dir
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.base_dir.join("agents")
    }

    pub fn cache_dir(&self) -> PathBuf {
        self.base_dir.join("cache")
    }

    pub fn settings_file(&self) -> PathBuf {
        self.base_dir.join("settings.json")
    }

    pub fn clear_cache(&self) {
        let cache_dir = self.cache_dir();
        if cache_dir.exists() {
            fs::remove_dir_all(&cache_dir).ok();
            fs::create_dir_all(&cache_dir).ok();
        }
    }
}
