use sqlx::SqlitePool;
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

    #[error("SQLx error: {0}")]
    SQLxError(#[from] sqlx::Error),
}

impl TaskError {
    fn to_display_message(&self) -> String {
        match self {
            TaskError::InvalidUUID(_) => "Invalid UUID".to_string(),
            TaskError::DatabaseError(_) => "Database error".to_string(),
            TaskError::SQLxError(_) => "Database error".to_string(),
        }
    }
}

struct TaskManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> TaskManager<'a> {
    fn new(db_pool: &'a SqlitePool) -> Result<Self, ()> {
        Ok(TaskManager { db_pool })
    }

    async fn update_task(
        &self,
        task_id: String,
        update_data: UpdatedTaskData,
    ) -> Result<Option<Task>, TaskError> {
        let uuid: Uuid = Uuid::parse_str(&task_id)?;

        let mut connection = self.db_pool.acquire().await.unwrap();

        let task = Task::load_by_id(uuid, &mut connection).await?;

        match task {
            None => return Ok(None),
            Some(mut task) => {
                task.update(update_data, &mut connection);
                task.save(&mut connection).await?;

                Ok(Some(task))
            }
        }
    }

    async fn _save_task(&self, task: Task) -> Result<Task, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        task.save(&mut connection).await?;

        Ok(task)
    }
}

#[tauri::command]
pub async fn update_task_command(
    task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
    db: State<'_, SqlitePool>,
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

    let task_manager = TaskManager::new(&db).unwrap();

    match task_manager.update_task(task_id, updated_task_data).await {
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
    use sqlx::sqlite::SqlitePool;
    use sqlx::Error;

    async fn create_in_memory_pool() -> Result<SqlitePool, Error> {
        let pool = SqlitePool::connect(":memory:").await?;
        Ok(pool)
    }

    async fn apply_migrations(pool: &SqlitePool) -> Result<(), Error> {
        let mut connection = pool.acquire().await.unwrap();

        sqlx::query(
            r#"
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
        )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            emoji TEXT,
            color TEXT,
            description TEXT,
            created_at_utc DATETIME NOT NULL,
            updated_at_utc DATETIME NOT NULL,
            archived_at_utc DATETIME
        )
            "#,
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn updates_a_task() {
        use super::*;

        let pool = create_in_memory_pool().await.unwrap();

        apply_migrations(&pool).await.unwrap();

        let manager = TaskManager::new(&pool).unwrap();
        let task = Task::new(
            "Test Task".to_string(),
            Some("This is a test task.".to_string()),
            None,
            None,
            None,
        );
        let task_id = task.id.clone();
        manager._save_task(task).await.unwrap();

        let updated_task_data = UpdatedTaskData {
            title: "Updated task".to_string(),
            description: Some("Updated description".to_string()),
            due_date: None,
            deadline: None,
            project_id: None,
        };

        let updated_task = manager
            .update_task(task_id.to_string(), updated_task_data)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated_task.title, "Updated task");
        assert_eq!(
            updated_task.description,
            Some("Updated description".to_string())
        );
    }
}
