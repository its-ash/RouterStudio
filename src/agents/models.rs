use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: String,
    pub inputs: HashMap<String, AgentInput>,
    pub model: AgentModel,
    pub output: AgentOutput,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInput {
    #[serde(rename = "type")]
    pub input_type: AgentInputType,
    pub required: Option<bool>,
    pub options: Option<Vec<String>>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AgentInputType {
    Text,
    Textarea,
    Select,
    Number,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentModel {
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    #[serde(rename = "type")]
    pub output_type: String,
}
