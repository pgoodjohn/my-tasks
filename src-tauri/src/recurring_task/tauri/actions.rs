use chrono::{DateTime, Utc};
use std::error::Error as StdError;
use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::recurring_task::manager::RecurringTaskManager;
use crate::recurring_task::Frequency;
use crate::repository::RepositoryProvider;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurringTaskData {
    task_id: String,
    frequency: String,
    interval: i32,
}

#[tauri::command]
pub async fn setup_recurring_task_command(
    data: CreateRecurringTaskData,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let task_id = Uuid::parse_str(&data.task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    let frequency: Frequency = data
        .frequency
        .try_into()
        .map_err(|e| format!("Invalid frequency: {}", e))?;

    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| format!("Failed to get task repository: {}", e))?;
    let mut recurring_task_repository = repository_provider
        .recurring_task_repository()
        .await
        .map_err(|e| format!("Failed to get recurring task repository: {}", e))?;

    let mut recurring_task_manager =
        RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);

    let recurring_task = recurring_task_manager
        .setup_recurring_task(task_id, frequency, data.interval)
        .await
        .map_err(|e| format!("Failed to setup recurring task: {}", e))?;

    serde_json::to_string(&recurring_task)
        .map_err(|e| format!("Failed to serialize recurring task: {}", e))
}
