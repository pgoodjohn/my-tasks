use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

use super::Project;
use crate::task::Task;

#[derive(Debug, Serialize)]
struct ProjectDetail {
    project: Project,
    tasks: Vec<Task>,
}

impl ProjectDetail {
    pub fn for_project_with_id(
        uuid: Uuid,
        connection: &Connection,
    ) -> Result<Option<Self>, String> {
        let project = Project::load_by_id(uuid, connection).unwrap();
        match project {
            None => return Ok(None),
            Some(project) => {
                let tasks = Task::load_for_project(project.id, connection).unwrap();
                return Ok(Some(ProjectDetail {
                    project: project,
                    tasks: tasks,
                }));
            }
        }
    }
}

#[tauri::command]
pub fn load_project_details_command(
    project_id: String,
    include_completed_tasks: bool,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!(
        "Running load project details command for project ID: {}, include_completed_tasks: {:?}",
        project_id,
        include_completed_tasks
    );
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    let project_detail = ProjectDetail::for_project_with_id(uuid, &conn).unwrap();

    match project_detail {
        Some(project_detail) => {
            if include_completed_tasks {
                return Ok(serde_json::to_string(&project_detail).unwrap());
            }

            let open_tasks: Vec<Task> = project_detail
                .tasks
                .into_iter()
                .filter(|task| task.completed_at_utc.is_none())
                .collect();

            let project_detail = ProjectDetail {
                project: project_detail.project,
                tasks: open_tasks,
            };

            return Ok(serde_json::to_string(&project_detail).unwrap());
        }
        None => {
            return Err("Project not found".to_string());
        }
    }
}
