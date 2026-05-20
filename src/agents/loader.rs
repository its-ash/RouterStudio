use super::Agent;
use crate::storage::Storage;
use anyhow::Result;
use std::fs;

pub struct AgentLoader {
    storage: Storage,
}

impl AgentLoader {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    pub fn load_installed_agents(&self) -> Vec<Agent> {
        let agents_dir = self.storage.agents_dir();
        
        if !agents_dir.exists() {
            return Vec::new();
        }

        let mut agents = Vec::new();

        if let Ok(entries) = fs::read_dir(agents_dir) {
            for entry in entries.flatten() {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(agent) = serde_yaml::from_str::<Agent>(&content) {
                        agents.push(agent);
                    }
                }
            }
        }

        agents
    }

    pub fn install_agent(&self, agent: &Agent) -> Result<()> {
        let agents_dir = self.storage.agents_dir();
        fs::create_dir_all(&agents_dir)?;

        let file_path = agents_dir.join(format!("{}.yaml", agent.id));
        let content = serde_yaml::to_string(agent)?;
        fs::write(file_path, content)?;

        Ok(())
    }

    pub fn uninstall_agent(&self, agent_id: &str) -> Result<()> {
        let file_path = self.storage.agents_dir().join(format!("{}.yaml", agent_id));
        fs::remove_file(file_path)?;
        Ok(())
    }
}
