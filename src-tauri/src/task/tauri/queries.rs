use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use chrono::Utc;

use crate::errors::handle_error;
use crate::task::manager::TaskManager;

#[tauri::command]
pub async fn load_task_activity_statistics_command(
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load task activity statistics command");

    let manager = TaskManager::new(&db_pool);

    let statistics = manager
        .load_statistics()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&statistics).unwrap())
}

#[tauri::command]
pub async fn load_tasks_inbox_command(db_pool: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks inbox command");

    let manager = TaskManager::new(&db_pool);

    let tasks = manager.load_inbox().await.map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_due_today_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks due today command");

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_due_before(Utc::now())
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_command(
    include_completed: bool,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running load tasks command - include_completed: {:?}",
        include_completed
    );

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_tasks(include_completed)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_subtasks_for_task_command(
    parent_task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;
    let task_manager = TaskManager::new(&db);

    let subtasks = task_manager
        .load_subtasks_for_task(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtasks).unwrap())
}

#[tauri::command]
pub async fn load_task_by_id_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;
    let manager = TaskManager::new(&db);

    let task = manager
        .load_by_id(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn load_completed_tasks_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load completed tasks command");

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_completed_tasks()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}
