use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::project::manager::ProjectsManager;

#[tauri::command]
pub async fn create_project_command(
    title: String,
    emoji: Option<String>,
    color: Option<String>,
    description: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running create project command for: {:?} | {:?}",
        title,
        description
    );

    let projects_manager = ProjectsManager::new(&db);
    let project = projects_manager
        .create_project(title, emoji, color, description)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn update_project_command(
    project_id: String,
    new_title: Option<String>,
    new_emoji: Option<String>,
    new_color: Option<String>,
    new_description: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running update project command for: {:?} | {:?}",
        new_title,
        new_description
    );

    let projects_manager = ProjectsManager::new(&db);

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project = projects_manager
        .update_project(
            project_uuid,
            new_title,
            new_emoji,
            new_color,
            new_description,
        )
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn archive_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running archive project command for project ID: {}",
        project_id
    );

    let projects_manager = ProjectsManager::new(&db);

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project = projects_manager
        .archive_project(project_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project).unwrap())
}
