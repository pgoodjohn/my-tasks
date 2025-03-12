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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecurringTaskData {
    task_id: String,
    frequency: String,
    interval: i32,
}

#[tauri::command]
pub async fn setup_recurring_task_command(
    data: CreateRecurringTaskData,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let task_id = Uuid::parse_str(&data.task_id).map_err(|e| handle_error(&e))?;
    let frequency: Frequency = data.frequency.parse().map_err(|e| handle_error(&e))?;

    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;
    let mut recurring_task_repository = repository_provider
        .recurring_task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let mut recurring_task_manager =
        RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);

    let recurring_task = recurring_task_manager
        .setup_recurring_task(task_id, frequency, data.interval)
        .await
        .map_err(|e| handle_error(&*e))?;

    serde_json::to_string(&recurring_task).map_err(|e| handle_error(&e))
}

#[tauri::command]
pub async fn update_recurring_task_command(
    data: UpdateRecurringTaskData,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let task_id = Uuid::parse_str(&data.task_id).map_err(|e| handle_error(&e))?;
    let frequency: Frequency = data.frequency.parse().map_err(|e| handle_error(&e))?;

    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;
    let mut recurring_task_repository = repository_provider
        .recurring_task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let mut recurring_task_manager =
        RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);

    let recurring_task = recurring_task_manager
        .update_recurring_task(task_id, frequency, data.interval)
        .await
        .map_err(|e| handle_error(&*e))?;

    serde_json::to_string(&recurring_task).map_err(|e| handle_error(&e))
}

#[tauri::command]
pub async fn delete_recurring_task_command(
    task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<(), String> {
    let task_id = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;
    let mut recurring_task_repository = repository_provider
        .recurring_task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let mut recurring_task_manager =
        RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);

    recurring_task_manager
        .delete_recurring_task(task_id)
        .await
        .map_err(|e| handle_error(&*e))
}
