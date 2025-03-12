use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::recurring_task::repository::RecurringTaskRepository;
use crate::repository::RepositoryProvider;

#[tauri::command]
pub async fn get_recurring_task_command(
    task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let task_id = Uuid::parse_str(&task_id).map_err(|e| format!("Invalid task ID: {}", e))?;
    let mut recurring_task_repository = repository_provider
        .recurring_task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let recurring_task = recurring_task_repository
        .find_by_task_id(task_id)
        .await
        .map_err(|e| handle_error(&e))?;

    serde_json::to_string(&recurring_task).map_err(|e| handle_error(&e))
}
