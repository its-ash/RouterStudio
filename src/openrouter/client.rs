use crate::agents::Agent;
use crate::render::OutputType;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub struct OpenRouterClient {
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn execute_agent(
        &self,
        agent: &Agent,
        inputs: &HashMap<String, String>,
        model_override: Option<&str>,
    ) -> Result<OutputType, String> {
        let prompt = self.render_prompt(&agent.prompt, inputs);
        let model = model_override.unwrap_or(&agent.model.model);

        let output_type = agent.output.output_type.as_str();

        match output_type {
            "image" => self.generate_image(model, &prompt).await,
            "markdown" => self.generate_text(model, &prompt).await.map(OutputType::Markdown),
            _ => self.generate_text(model, &prompt).await.map(OutputType::Text),
        }
    }

    fn render_prompt(&self, template: &str, inputs: &HashMap<String, String>) -> String {
        let mut result = template.to_string();

        for (key, value) in inputs {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }

        result
    }

    async fn generate_text(&self, model: &str, prompt: &str) -> Result<String, String> {
        let client = reqwest::Client::new();

        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            stream: false,
        };

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No response from API".to_string())
    }

    async fn generate_image(&self, _model: &str, prompt: &str) -> Result<OutputType, String> {
        let url = format!("https://image.pollinations.ai/prompt/{}", urlencoding::encode(prompt));
        
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch image: {}", e))?;

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read image bytes: {}", e))?;

        let image = image::load_from_memory(&bytes)
            .map_err(|e| format!("Failed to load image: {}", e))?;

        Ok(OutputType::Image(Arc::new(image)))
    }
}
