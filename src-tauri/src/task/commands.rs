use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use serde::Serialize;
use tauri::State;
use thiserror::Error;
use uuid::Uuid;

use super::UpdatedTaskData;
use crate::task::Task;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    command: String,
    message: String,
    display_message: String,
}

impl ErrorResponse {
    fn new(command: String, message: String, display_message: String) -> Self {
        ErrorResponse {
            command,
            message: message,
            display_message: display_message,
        }
    }
}

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Invalid UUID: {0}")]
    InvalidUUID(#[from] uuid::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
}

impl TaskError {
    fn to_display_message(&self) -> String {
        match self {
            TaskError::InvalidUUID(_) => "Invalid UUID".to_string(),
            TaskError::DatabaseError(_) => "Database error".to_string(),
        }
    }
}

struct TaskManager {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl TaskManager {
    fn new(connection: PooledConnection<SqliteConnectionManager>) -> Result<Self, ()> {
        Ok(TaskManager { connection })
    }

    fn update_task(
        &self,
        task_id: String,
        update_data: UpdatedTaskData,
    ) -> Result<Option<Task>, TaskError> {
        let uuid = Uuid::parse_str(&task_id)?;

        let task = Task::load_by_id(uuid, &self.connection)?;

        match task {
            None => return Ok(None),
            Some(mut task) => {
                task.update(update_data)?;
                task.save(&self.connection)?;

                Ok(Some(task))
            }
        }
    }
}

#[tauri::command]
pub fn update_task_command(
    task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
    db: State<Pool<SqliteConnectionManager>>,
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

    let conn = db.get().unwrap(); // Get a connection from the pool
    let task_manager = TaskManager::new(conn).unwrap();

    match task_manager.update_task(task_id, updated_task_data) {
        Ok(task) => Ok(serde_json::to_string(&task).unwrap()),
        Err(e) => {
            let error = ErrorResponse::new(
                "update_task_command".to_string(),
                e.to_string(),
                e.to_display_message(),
            );
            log::error!("Error updating task: {:?}", error);
            Err(serde_json::to_string(&error).unwrap())
        }
    }
}
