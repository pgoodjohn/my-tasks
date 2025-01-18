use chrono::{DateTime, Utc};
use r2d2::{Error, Pool};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use serde::Serialize;
use tauri::State;
use thiserror::Error;
use uuid::Uuid;

use super::UpdatedTaskData;
use crate::task::Task;

#[derive(Debug)]
struct ErrorResponse {
    command: String,
    message: String,
    display_message: String,
}

impl serde::Serialize for ErrorResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.display_message.to_string().as_ref())
    }
}

impl ErrorResponse {
    fn new(command: String, message: String) -> Self {
        ErrorResponse {
            command,
            message: message.clone(),
            display_message: message,
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

fn update_task(
    task_id: String,
    update_data: UpdatedTaskData,
    connection: &rusqlite::Connection,
) -> Result<Option<Task>, TaskError> {
    let uuid = Uuid::parse_str(&task_id)?;

    let task = Task::load_by_id(uuid, &connection)?;

    match task {
        None => return Ok(None),
        Some(mut task) => {
            task.update(update_data)?;
            task.save(&connection)?;

            Ok(Some(task))
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

    match update_task(task_id, updated_task_data, &conn) {
        Ok(task) => Ok(serde_json::to_string(&task).unwrap()),
        Err(e) => {
            let error = ErrorResponse::new("update_task_command".to_string(), e.to_string());
            log::error!("Error updating task: {:?}", error);
            Err(serde_json::to_string(&error).unwrap())
        }
    }
}
