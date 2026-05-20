use super::Agent;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistry {
    pub version: i32,
    pub agents: Vec<AgentMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub file: String,
    pub tags: Option<Vec<String>>,
}

impl AgentRegistry {
    pub async fn fetch(url: &str) -> Result<Vec<Agent>, String> {
        let response = reqwest::get(url)
            .await
            .map_err(|e| format!("Failed to fetch registry: {}", e))?;

        let registry: AgentRegistry = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse registry: {}", e))?;

        let mut agents = Vec::new();

        for metadata in registry.agents {
            let base_url = url.rsplit_once('/').map(|(b, _)| b).unwrap_or("");
            let agent_url = format!("{}/{}", base_url, metadata.file);

            if let Ok(agent_response) = reqwest::get(&agent_url).await {
                if let Ok(agent_text) = agent_response.text().await {
                    if let Ok(agent) = serde_yaml::from_str::<Agent>(&agent_text) {
                        agents.push(agent);
                    }
                }
            }
        }

        Ok(agents)
    }
}
