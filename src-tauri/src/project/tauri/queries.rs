use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::project::manager::ProjectsManager;
use crate::repository::RepositoryProvider;

#[tauri::command]
pub async fn load_favorite_projects_command(
    repository_provider: State<'_, RepositoryProvider>,
) -> Result<String, String> {
    log::debug!("Running load favorite projects command");

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

    let projects = projects_manager
        .load_favorites()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&projects).unwrap())
}

#[tauri::command]
pub async fn load_projects_command(
    repository_provider: State<'_, RepositoryProvider>,
    show_archived_projects: bool,
) -> Result<String, String> {
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

    let projects = projects_manager
        .load_all(show_archived_projects)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&projects).unwrap())
}

#[tauri::command]
pub async fn load_project_details_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
    include_completed_tasks: bool,
) -> Result<String, String> {
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

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project_detail = projects_manager
        .load_project_detail(project_uuid, include_completed_tasks)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project_detail).unwrap())
}

#[tauri::command]
pub async fn count_open_tasks_for_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
) -> Result<i64, String> {
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

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let count = projects_manager
        .count_open_tasks(project_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(count)
}

#[tauri::command]
pub async fn add_favorite_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
) -> Result<String, String> {
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

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project = projects_manager
        .add_favorite(project_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn remove_favorite_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
) -> Result<String, String> {
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

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project = projects_manager
        .remove_favorite(project_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}
