use super::Agent;
use anyhow::Result;
use serde::{Deserialize, Serialize};

fn registry_base_url(url: &str) -> String {
    url.rsplit_once('/')
        .map(|(base, _)| base.to_string())
        .unwrap_or_default()
}

fn resolve_agent_url(registry_url: &str, file_path: &str) -> String {
    if file_path.starts_with("http://") || file_path.starts_with("https://") {
        return file_path.to_string();
    }

    let base = registry_base_url(registry_url);
    if base.is_empty() {
        file_path.to_string()
    } else {
        format!("{}/{}", base, file_path.trim_start_matches('/'))
    }
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

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

        let status = response.status();
        if !status.is_success() {
            return Err(format!("Failed to fetch registry: HTTP {}", status));
        }

        let registry_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read registry body: {}", e))?;

        let registry: AgentRegistry = serde_json::from_str(&registry_text)
            .or_else(|_| serde_yaml::from_str(&registry_text))
            .map_err(|e| format!("Failed to parse registry: {}", e))?;

        let mut agents = Vec::new();
        let mut failed_count = 0usize;

        for metadata in registry.agents {
            let primary_url = resolve_agent_url(url, &metadata.file);
            let mut urls = vec![primary_url.clone()];

            let primary_name = file_name(&metadata.file);
            if !metadata.file.eq(primary_name) {
                urls.push(resolve_agent_url(url, primary_name));
            }

            let mut loaded = false;
            for agent_url in urls {
                if let Ok(agent_response) = reqwest::get(&agent_url).await {
                    if agent_response.status().is_success() {
                        if let Ok(agent_text) = agent_response.text().await {
                            if let Ok(agent) = serde_yaml::from_str::<Agent>(&agent_text) {
                                agents.push(agent);
                                loaded = true;
                                break;
                            }
                        }
                    }
                }
            }

            if !loaded {
                failed_count += 1;
            }
        }

        if agents.is_empty() && failed_count > 0 {
            return Err("Failed to load agent files from registry entries".to_string());
        }

        Ok(agents)
    }
}
