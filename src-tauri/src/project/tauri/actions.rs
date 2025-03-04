use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::project::manager::ProjectsManager;
use crate::project::Project;
use crate::repository::RepositoryProvider;

#[tauri::command]
pub async fn create_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    title: String,
    description: Option<String>,
) -> Result<Project, String> {
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

    projects_manager
        .create_project(title, description)
        .await
        .map_err(|e| handle_error(&*e))
}

#[tauri::command]
pub async fn update_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
    title: String,
    emoji: Option<String>,
    color: Option<String>,
    description: Option<String>,
) -> Result<Project, String> {
    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
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

    projects_manager
        .update_project(project_uuid, title, emoji, color, description)
        .await
        .map_err(|e| handle_error(&*e))
}

#[tauri::command]
pub async fn archive_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
) -> Result<Project, String> {
    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
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

    projects_manager
        .archive_project(project_uuid)
        .await
        .map_err(|e| handle_error(&*e))
}
