use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::project::manager::ProjectsManager;
use crate::task::manager::TaskManager;
use crate::task::{CreateTaskData, UpdatedTaskData};
use crate::RepositoryProvider;

#[tauri::command]
pub async fn create_task_command(
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let create_task_data = CreateTaskData {
        title,
        description,
        due_at_utc: due_date,
        project_id,
    };

    log::debug!("Running update task command for: | {:?}", create_task_data);

    let task_manager = TaskManager::new(&repository_provider);

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
    project_id: Option<String>,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let updated_task_data = UpdatedTaskData {
        title,
        description,
        due_date,
        project_id,
    };

    log::debug!(
        "Running update task command for: {:?} | {:?}",
        task_id,
        updated_task_data,
    );

    let task_manager = TaskManager::new(&repository_provider);

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
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running delete task command for card ID: {}", task_id);

    let task_manager = TaskManager::new(&repository_provider);
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
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running complete task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let manager = TaskManager::new(&repository_provider);

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
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;

    let task_manager = TaskManager::new(&repository_provider);

    let parent_task = task_manager
        .load_by_id(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?
        .ok_or_else(|| "Parent task not found".to_string())?;

    let create_task_data = CreateTaskData {
        title,
        description,
        due_at_utc: due_date,
        project_id: parent_task.project_id.map(|id| id.to_string()),
    };

    let subtask = task_manager
        .create_subtask_for_task(parent_task, create_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtask).unwrap())
}

#[tauri::command]
pub async fn promote_task_to_project_command(
    task_id: String,
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let task_id = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;
    let task_manager = TaskManager::new(&repository_provider);
    let task = task_manager
        .load_task(task_id)
        .await
        .map_err(|e| handle_error(&*e))?;

    let mut project_repository = repository_provider
        .inner()
        .project_repository()
        .await
        .map_err(|e| handle_error(&e))?;
    let mut task_repository = repository_provider
        .inner()
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;
    let mut projects_manager = ProjectsManager::new(&mut project_repository, &mut task_repository);

    let project = projects_manager
        .create_project(task.title.clone(), task.description.clone())
        .await
        .map_err(|e| handle_error(&*e))?;

    task_manager
        .move_subtasks_to_project(task_id, project.id)
        .await
        .map_err(|e| handle_error(&e))?;

    task_manager
        .archive_task(task_id)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}
