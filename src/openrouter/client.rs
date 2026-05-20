use crate::agents::{Agent, AgentInputType};
use crate::render::OutputType;
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
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

#[derive(Serialize)]
struct MultiModalChatRequest {
    model: String,
    messages: Vec<MultiModalMessage>,
    stream: bool,
}

#[derive(Serialize)]
struct MultiModalMessage {
    role: String,
    content: Vec<MultiModalContent>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum MultiModalContent {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

#[derive(Serialize)]
struct ImageUrl {
    url: String,
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
        let image_files = self.collect_file_inputs(agent, inputs)?;

        let output_type = agent.output.output_type.as_str();

        match output_type {
            "image" => {
                if image_files.is_empty() {
                    self.generate_image(model, &prompt).await
                } else {
                    let enriched_prompt = self
                        .enrich_prompt_with_logo(model, &prompt, &image_files)
                        .await
                        .unwrap_or(prompt);
                    self.generate_image(model, &enriched_prompt).await
                }
            }
            "markdown" => self.generate_text(model, &prompt).await.map(OutputType::Markdown),
            _ => self.generate_text(model, &prompt).await.map(OutputType::Text),
        }
    }

    fn collect_file_inputs(
        &self,
        agent: &Agent,
        inputs: &HashMap<String, String>,
    ) -> Result<Vec<String>, String> {
        let mut files = Vec::new();

        for (name, input) in &agent.inputs {
            if matches!(input.input_type, AgentInputType::File) {
                let value = inputs.get(name).map(|v| v.trim()).unwrap_or("");
                if value.is_empty() {
                    if input.required.unwrap_or(false) {
                        return Err(format!("Missing required file input: {}", name));
                    }
                    continue;
                }

                if !Path::new(value).exists() {
                    return Err(format!("File not found for input '{}': {}", name, value));
                }

                files.push(value.to_string());
            }
        }

        Ok(files)
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

    async fn enrich_prompt_with_logo(
        &self,
        model: &str,
        base_prompt: &str,
        image_paths: &[String],
    ) -> Result<String, String> {
        let mut content = vec![MultiModalContent::Text {
            text: format!(
                "You are preparing a single final image-generation prompt. Use the uploaded logo image as brand reference and keep visual identity aligned. Return only the final prompt text. Base request:\n{}",
                base_prompt
            ),
        }];

        for path in image_paths {
            let bytes = fs::read(path).map_err(|e| format!("Failed reading file '{}': {}", path, e))?;
            let mime = detect_image_mime(path)
                .ok_or_else(|| format!("Unsupported file type for '{}'. Use png, jpg, jpeg, webp, gif", path))?;
            let b64 = general_purpose::STANDARD.encode(bytes);
            let data_url = format!("data:{};base64,{}", mime, b64);
            content.push(MultiModalContent::ImageUrl {
                image_url: ImageUrl { url: data_url },
            });
        }

        let request = MultiModalChatRequest {
            model: if model == "pollinations" {
                "openai/gpt-4.1-mini".to_string()
            } else {
                model.to_string()
            },
            messages: vec![MultiModalMessage {
                role: "user".to_string(),
                content,
            }],
            stream: false,
        };

        let response = reqwest::Client::new()
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Multimodal request failed: {}", e))?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse multimodal response: {}", e))?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No multimodal response from API".to_string())
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

fn detect_image_mime(path: &str) -> Option<&'static str> {
    let ext = Path::new(path).extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "png" => Some("image/png"),
        "jpg" | "jpeg" => Some("image/jpeg"),
        "webp" => Some("image/webp"),
        "gif" => Some("image/gif"),
        _ => None,
    }
}
