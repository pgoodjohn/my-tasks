use crate::repository::RepositoryProvider;
use crate::task::manager::TaskManager;
use tauri::State;
use uuid::Uuid;

use chrono::Utc;

use crate::errors::handle_error;

#[tauri::command]
pub async fn load_task_activity_statistics_command(
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running load task activity statistics command");

    let manager = TaskManager::new(&repository_provider);

    let statistics = manager
        .load_statistics()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&statistics).unwrap())
}

#[tauri::command]
pub async fn load_tasks_inbox_command(
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running load tasks inbox command");

    let manager = TaskManager::new(&repository_provider);

    let tasks = manager.load_inbox().await.map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_due_today_command(
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running load tasks due today command");

    let manager = TaskManager::new(&repository_provider);

    let tasks = manager
        .load_due_before(Utc::now())
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn load_tasks_command(
    repository_provider: State<'_, RepositoryProvider>,
    include_completed: bool,
) -> Result<String, String> {
    log::debug!(
        "Running load tasks command - include_completed: {:?}",
        include_completed
    );

    let manager = TaskManager::new(&repository_provider);

    let tasks = manager
        .load_tasks(include_completed)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_subtasks_for_task_command(
    parent_task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;
    let task_manager = TaskManager::new(&repository_provider);

    let subtasks = task_manager
        .load_subtasks_for_task(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtasks).unwrap())
}

#[tauri::command]
pub async fn load_completed_subtasks_for_task_command(
    parent_task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;
    let task_manager = TaskManager::new(&repository_provider);

    let subtasks = task_manager
        .load_completed_subtasks_for_task(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtasks).unwrap())
}

#[tauri::command]
pub async fn load_task_by_id_command(
    task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;
    let manager = TaskManager::new(&repository_provider);

    let task = manager
        .load_by_id(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn load_completed_tasks_command(
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running load completed tasks command");

    let manager = TaskManager::new(&repository_provider);

    let tasks = manager
        .load_completed_tasks()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command(rename_all = "camelCase")]
pub async fn load_tasks_by_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
    include_completed: bool,
) -> Result<String, String> {
    log::debug!(
        "Running load tasks by project command - project_id: {:?}, include_completed: {:?}",
        project_id,
        include_completed
    );

    let manager = TaskManager::new(&repository_provider);
    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let tasks = manager
        .load_tasks_by_project(project_uuid, include_completed)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}
