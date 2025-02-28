use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use crate::errors::handle_error;
use crate::project::manager::ProjectsManager;

#[tauri::command]
pub async fn load_favorite_projects_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load favorite projects command");

    let projects_manager = ProjectsManager::new(&db);

    let projects = projects_manager
        .load_favorites()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&projects).unwrap())
}

#[tauri::command]
pub async fn load_projects_command(
    show_archived_projects: bool,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running list projects command");
    let projects_manager = ProjectsManager::new(&db);

    let projects = projects_manager
        .load_projects(show_archived_projects)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&projects).unwrap())
}

#[tauri::command]
pub async fn load_project_details_command(
    project_id: String,
    include_completed_tasks: bool,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running load project details command for project ID: {}, include_completed_tasks: {:?}",
        project_id,
        include_completed_tasks
    );

    let projects_manager = ProjectsManager::new(&db);

    let project_uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let project_details = projects_manager
        .load_project_details(project_uuid, include_completed_tasks)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&project_details).unwrap())
}

#[tauri::command]
pub async fn count_open_tasks_for_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running count open tasks for project command for project ID: {}",
        project_id
    );

    let manager = ProjectsManager::new(&db);

    let uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let open_tasks_count = manager
        .count_open_tasks(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(open_tasks_count.to_string())
}

#[tauri::command]
pub async fn add_favorite_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running favorite project command for project ID: {}",
        project_id
    );

    let manager = ProjectsManager::new(&db);

    let uuid = Uuid::parse_str(&project_id).map_err(|e| handle_error(&e))?;

    let favorite_project = manager
        .add_favorite(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&favorite_project).unwrap())
}

#[tauri::command]
pub async fn remove_favorite_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running favorite project command for project ID: {}",
        project_id
    );

    let manager = ProjectsManager::new(&db);

    let uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    let favorite_project = manager.remove_favorite(uuid).await.unwrap();

    Ok(serde_json::to_string(&favorite_project).unwrap())
}
