use regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub mod tauri;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model: String,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:11434".to_string(),
            model: "deepseek-r1".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub thinking: Option<String>,
    pub done: bool,
}

pub async fn get_task_prioritization(
    tasks_text: String,
    config: &OllamaConfig,
) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let prompt = format!(
        "Here are my current tasks. Please help me prioritize them and suggest which ones I should focus on first. Consider urgency, importance, and dependencies:\n\n{}",
        tasks_text
    );

    let request = OllamaRequest {
        model: config.model.clone(),
        prompt,
    };

    log::debug!("Ollama request: {:?}", request);

    // Get the response text
    let response_text = client
        .post(format!("{}/api/generate", config.base_url))
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

    log::debug!("Raw response text: {}", response_text);

    // The response is a series of JSON objects, one per line
    // We'll collect all responses and combine them
    let mut full_response = String::new();
    let mut model_name = String::new();

    for line in response_text.lines() {
        if let Ok(resp) = serde_json::from_str::<OllamaResponse>(line) {
            full_response.push_str(&resp.response);
            if model_name.is_empty() {
                model_name = resp.model;
            }
        }
    }

    log::debug!("Final response text: {}", full_response);

    // Extract thinking process and final response
    let (thinking, response) = if let Some(thinking_content) = extract_thinking(&full_response) {
        (
            Some(thinking_content.to_string()),
            remove_thinking(&full_response),
        )
    } else {
        (None, full_response)
    };

    Ok(OllamaResponse {
        model: model_name,
        response: response.trim().to_string(),
        thinking,
        done: true,
    })
}

fn extract_thinking(text: &str) -> Option<&str> {
    let start_tag = "<think>";
    let end_tag = "</think>";

    if let (Some(start), Some(end)) = (text.find(start_tag), text.find(end_tag)) {
        let content_start = start + start_tag.len();
        if content_start < end {
            Some(&text[content_start..end])
        } else {
            None
        }
    } else {
        None
    }
}

fn remove_thinking(text: &str) -> String {
    let re = regex::Regex::new(r"(?s)<think>.*?</think>").unwrap();
    re.replace_all(text, "").to_string()
}
