use crate::project::manager::ProjectsManager;
use crate::project::repository::RepositoryProvider;
use crate::project::Project;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    title: String,
) -> Result<Project, String> {
    let projects_manager = ProjectsManager::new(&repository_provider);
    projects_manager
        .create_project(title)
        .await
        .map_err(|e| e.to_string())
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
    let projects_manager = ProjectsManager::new(&repository_provider);

    projects_manager
        .update_project(project_uuid, title, emoji, color, description)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn archive_project_command(
    repository_provider: State<'_, RepositoryProvider>,
    project_id: String,
) -> Result<Project, String> {
    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let projects_manager = ProjectsManager::new(&repository_provider);

    projects_manager
        .archive_project(project_uuid)
        .await
        .map_err(|e| e.to_string())
}
