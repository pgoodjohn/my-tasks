use reqwest::Client;
use serde::{Deserialize, Serialize};

pub mod tauri;

const OLLAMA_BASE_URL: &str = "http://localhost:11434";

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub done: bool,
}

pub async fn get_task_prioritization(
    tasks_text: String,
) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let prompt = format!(
        "Here are my current tasks. Please help me prioritize them and suggest which ones I should focus on first. Consider urgency, importance, and dependencies:\n\n{}",
        tasks_text
    );

    let request = OllamaRequest {
        model: "deepseek-r1".to_string(),
        prompt,
    };

    log::debug!("Ollama request: {:?}", request);

    // Get the response text
    let response_text = client
        .post(format!("{}/api/generate", OLLAMA_BASE_URL))
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

    Ok(OllamaResponse {
        model: model_name,
        response: full_response,
        done: true,
    })
}
