use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use tauri::State;
use thiserror::Error;
use uuid::Uuid;

use super::UpdatedTaskData;
use crate::commands::ErrorResponse;
use crate::task::Task;

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

struct TaskManager<'a> {
    connection: &'a Connection,
}

impl<'a> TaskManager<'a> {
    fn new(connection: &'a Connection) -> Result<Self, ()> {
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
                task.update(update_data, &self.connection)?;
                task.save(&self.connection)?;

                Ok(Some(task))
            }
        }
    }

    fn _save_task(&self, task: Task) -> Result<Task, TaskError> {
        task.save(&self.connection)?;

        Ok(task)
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

    let connection = db.get().unwrap(); // Get a connection from the pool
    let task_manager = TaskManager::new(&connection).unwrap();

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

#[cfg(test)]
mod tests {
    #[test]
    fn updates_a_task() {
        use super::*;
        use rusqlite::Connection;

        fn _setup_in_memory_db() -> Connection {
            let conn = Connection::open_in_memory().unwrap();
            conn.execute(
                "
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            project_id TEXT,
            due_at_utc DATETIME,
            deadline_at_utc DATETIME,
            created_at_utc DATETIME NOT NULL,
            completed_at_utc DATETIME,
            updated_at_utc DATETIME NOT NULL
        );",
                [],
            )
            .unwrap();

            conn.execute(
                "
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            emoji TEXT,
            color TEXT,
            description TEXT,
            created_at_utc DATETIME NOT NULL,
            updated_at_utc DATETIME NOT NULL,
            archived_at_utc DATETIME
        );",
                [],
            )
            .unwrap();

            conn
        }

        let conn = _setup_in_memory_db();

        let manager = TaskManager::new(&conn).unwrap();
        let task = Task::new(
            "Test Task".to_string(),
            Some("This is a test task.".to_string()),
            None,
            None,
            None,
        );
        let task_id = task.id.clone();
        manager._save_task(task).unwrap();

        let updated_task_data = UpdatedTaskData {
            title: "Updated task".to_string(),
            description: Some("Updated description".to_string()),
            due_date: None,
            deadline: None,
            project_id: None,
        };

        let updated_task = manager
            .update_task(task_id.to_string(), updated_task_data)
            .unwrap()
            .unwrap();

        assert_eq!(updated_task.title, "Updated task");
        assert_eq!(
            updated_task.description,
            Some("Updated description".to_string())
        );
    }
}
