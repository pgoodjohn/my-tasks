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
        r#"REMEMBER: NO HALLUCINATING. STICK TO THE INFO YOU HAVE.

You are an expert task prioritizer. You will be given a list of tasks with the following details:

    Task descriptions
    Associated project (if any)
    Due dates
    Creation dates
    Last updated dates
    Progress information (not provided but assumed to be available in the task context)

Your job is to analyze this list of tasks and provide a prioritized list based on the following criteria:

    Due Date: Tasks that are closer to their due date should be prioritized higher.
    Project Context: If the task is part of a larger project, prioritize those tasks to ensure progress on the overall project.
    Created Date: Tasks created more recently may need attention sooner, especially if there is no due date.
    Last Updated Date: If a task hasn't been updated recently, it may require more immediate attention.

Additionally, you should take into account if any tasks have dependencies or subtasks, such as when one task is part of a larger project.

Here is the list of tasks you'll need to prioritize:

{}"#,
        tasks_text
    );

    let request = OllamaRequest {
        model: config.model.clone(),
        prompt,
    };

    // Get the response text
    let response_text = client
        .post(format!("{}/api/generate", config.base_url))
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

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

    // Extract thinking process and final response
    let (thinking, response) = if let Some(thinking_content) = extract_thinking(&full_response) {
        (
            Some(thinking_content.to_string()),
            remove_thinking(&full_response),
        )
    } else {
        (None, full_response)
    };

    log::debug!("Ollama request: {:?}", request);
    log::debug!("Thinking: {:?}", thinking);
    log::debug!("Final response text: {}", response);

    Ok(OllamaResponse {
        model: model_name,
        response: response.trim().to_string(),
        thinking,
        done: true,
    })
}

pub async fn get_quick_task(
    tasks_text: String,
    config: &OllamaConfig,
) -> Result<OllamaResponse, Box<dyn std::error::Error>> {
    let client = Client::new();

    let prompt = format!(
        r#"REMEMBER: NO HALLUCINATING. STICK TO THE INFO YOU HAVE.

You are a task prioritizer focused on helping someone find a quick task to complete within the next 30 minutes. You will be given a list of tasks with details such as:

    Task descriptions
    Associated projects (if any)
    Due dates
    Creation dates
    Last updated dates
    Progress information (not provided but assumed to be available in the task context)

Your goal is to identify which task is the most feasible to complete in the next 30 minutes. Consider the following criteria:

    Task Duration: Choose tasks that seem quick and achievable within a short time frame (30 minutes).
    Due Date: Tasks that are approaching their due date should be prioritized, especially if they are simple and can be completed in a short time.
    Current Progress: Tasks that are in-progress or almost finished should be prioritized if they can be completed quickly.
    Task Simplicity: If the task description suggests it is simple and straightforward, prioritize it for completion within the next 30 minutes.

Provide the task that seems easiest and quickest to accomplish based on the input below:

Input:

    {}

Output:
Identify and recommend the task that can realistically be completed in the next 30 minutes based on the provided details."#,
        tasks_text
    );

    let request = OllamaRequest {
        model: config.model.clone(),
        prompt,
    };

    // Get the response text
    let response_text = client
        .post(format!("{}/api/generate", config.base_url))
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

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

    // Extract thinking process and final response
    let (thinking, response) = if let Some(thinking_content) = extract_thinking(&full_response) {
        (
            Some(thinking_content.to_string()),
            remove_thinking(&full_response),
        )
    } else {
        (None, full_response)
    };

    log::debug!("Ollama request: {:?}", request);
    log::debug!("Thinking: {:?}", thinking);
    log::debug!("Final response text: {}", response);

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
