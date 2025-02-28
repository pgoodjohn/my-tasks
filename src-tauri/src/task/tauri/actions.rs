use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use chrono::{DateTime, Utc};

use crate::errors::handle_error;
use crate::task::{manager::TaskManager, CreateTaskData, UpdatedTaskData};

#[tauri::command]
pub async fn create_task_command(
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let due_at_utc = match due_date {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let deadline_at_utc = match deadline {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let project_id_uuid = match project_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|e| handle_error(&e))?),
        None => None,
    };

    let create_task_data = CreateTaskData {
        title,
        description,
        due_at_utc,
        deadline_at_utc,
        project_id: project_id_uuid,
    };

    log::debug!("Running update task command for: | {:?}", create_task_data);

    let task_manager = TaskManager::new(&db);

    let task = task_manager
        .create_task(create_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn update_task_command(
    task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let updated_task_data = UpdatedTaskData {
        title,
        description,
        due_date,
        deadline,
        project_id,
    };

    log::debug!(
        "Running update task command for: {:?} | {:?}",
        task_id,
        updated_task_data,
    );

    let task_manager = TaskManager::new(&db);

    let uuid: Uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let task = task_manager
        .update_task(uuid, updated_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn delete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running delete task command for card ID: {}", task_id);

    let task_manager = TaskManager::new(&db);
    let task_uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    task_manager
        .delete_task(task_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(format!("Task with ID {} deleted successfully", &task_id))
}

#[tauri::command]
pub async fn complete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running complete task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let manager = TaskManager::new(&db);

    manager
        .complete_task(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok("{}".to_string())
}

#[tauri::command]
pub async fn create_subtask_for_task_command(
    parent_task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let due_at_utc = match due_date {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;

    let task_manager = TaskManager::new(&db);

    let parent_task = task_manager
        .load_by_id(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?
        .unwrap(); // TODO: Remove this unwrap is parent task cant be found anymore (shouldnt happen really)

    let create_task_data = CreateTaskData {
        title,
        project_id: parent_task.project.as_ref().map(|p| p.id),
        description,
        due_at_utc,
        deadline_at_utc: parent_task.deadline_at_utc,
    };

    let subtask = task_manager
        .create_subtask_for_task(parent_task, create_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtask).unwrap())
}
