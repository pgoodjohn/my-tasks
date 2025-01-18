use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use tauri::State;

use super::Project;
use crate::commands::ErrorResponse;

struct ProjectsManager<'a> {
    connection: &'a Connection,
}

impl<'a> ProjectsManager<'a> {
    fn new(connection: &'a Connection) -> Result<Self, ()> {
        Ok(ProjectsManager { connection })
    }

    fn load_projects(&self, show_archived_projects: bool) -> Result<Vec<Project>, ()> {
        match show_archived_projects {
            true => {
                let projects = Project::list_all_projects(&self.connection)?;
                return Ok(projects);
            }
            false => {
                let projects = Project::list_not_archived_projects(&self.connection)?;
                return Ok(projects);
            }
        }
    }
}

#[tauri::command]
pub fn load_projects_command(
    show_archived_projects: bool,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running list projects command");
    let connection = db.get().unwrap(); // Get a connection from the pool
    let projects_manager = ProjectsManager::new(&connection).unwrap();

    match projects_manager.load_projects(show_archived_projects) {
        Ok(projects) => Ok(serde_json::to_string(&projects).unwrap()),
        Err(_) => {
            let error = ErrorResponse::new(
                "load_projects_command".to_string(),
                "Failed to load projects".to_string(),
                "Failed to load projects".to_string(),
            );
            Err(serde_json::to_string(&error).unwrap())
        }
    }
}
