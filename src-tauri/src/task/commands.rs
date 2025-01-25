use std::collections::HashMap;

use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite, SqlitePool};
use tauri::State;
use thiserror::Error;
use uuid::Uuid;

use super::{CreateTaskData, UpdatedTaskData};
use crate::commands::ErrorResponse;
use crate::project::Project;
use crate::task::Task;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Invalid UUID: {0}")]
    InvalidUUID(#[from] uuid::Error),

    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("SQLx error: {0}")]
    SQLxError(#[from] sqlx::Error),

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Task not found")]
    TaskNotFound,
}

impl TaskError {
    fn to_display_message(&self) -> String {
        match self {
            TaskError::InvalidUUID(_) => "Invalid UUID".to_string(),
            TaskError::DatabaseError(_) => "Database error".to_string(),
            TaskError::SQLxError(_) => "Database error".to_string(),
            TaskError::ProjectNotFound => "Project not found".to_string(),
            TaskError::TaskNotFound => "Task not found".to_string(),
        }
    }
}

pub struct TaskManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> TaskManager<'a> {
    pub fn new(db_pool: &'a SqlitePool) -> Result<Self, ()> {
        Ok(TaskManager { db_pool })
    }

    pub async fn create_task(&self, create_task_data: CreateTaskData) -> Result<Task, TaskError> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let project = match create_task_data.project_id {
            Some(id) => match self.load_project_by_uuid(&mut connection, id).await {
                Ok(project) => project,
                Err(_) => return Err(TaskError::ProjectNotFound),
            },
            None => None,
        };

        let task = Task::new(
            create_task_data.title,
            create_task_data.description,
            project,
            create_task_data.due_at_utc,
            create_task_data.deadline_at_utc,
        );
        task.create_record(&mut connection).await?;

        Ok(task)
    }

    async fn load_project_by_uuid(
        &self,
        connection: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
        id: Uuid,
    ) -> Result<Option<Project>, TaskError> {
        let project = Project::load_by_id(id, connection)
            .await
            .map_err(|_| TaskError::ProjectNotFound)?;
        Ok(project)
    }

    pub async fn update_task(
        &self,
        task_id: Uuid,
        update_data: UpdatedTaskData,
    ) -> Result<Option<Task>, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let task = Task::load_by_id(task_id, &mut connection).await.unwrap();

        match task {
            None => return Ok(None),
            Some(mut task) => {
                let _ = task.update(update_data, &mut connection).await.unwrap();
                task.update_record(&mut connection).await?;

                Ok(Some(task))
            }
        }
    }

    async fn delete_task(&self, task_id: Uuid) -> Result<(), TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let task = Task::load_by_id(task_id, &mut connection).await.unwrap();

        match task {
            Some(t) => {
                t.delete_record(&mut connection).await?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    async fn load_tasks(&self, include_completed: bool) -> Result<Vec<Task>, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let tasks = Task::load_filtered_by_completed(include_completed, &mut connection)
            .await
            .unwrap();

        return Ok(tasks);
    }

    async fn complete_task(&self, task_id: Uuid) -> Result<(), TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let task = Task::load_by_id(task_id, &mut connection).await.unwrap();

        match task {
            None => Err(TaskError::TaskNotFound),
            Some(mut t) => {
                t.completed_at_utc = Some(Utc::now());
                t.update_record(&mut connection).await.unwrap();

                return Ok(());
            }
        }
    }

    async fn load_inbox(&self) -> Result<Vec<Task>, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let tasks = Task::load_inbox(&mut connection).await.unwrap();

        Ok(tasks)
    }

    async fn load_due_before(&self, date: DateTime<Utc>) -> Result<Vec<Task>, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let tasks = Task::load_due_before(date, &mut connection).await.unwrap();

        Ok(tasks)
    }

    async fn load_with_deadline(&self) -> Result<Vec<Task>, TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let tasks = Task::load_with_deadlines(&mut connection).await.unwrap();

        Ok(tasks)
    }

    async fn load_statistics(&self) -> Result<PeriodTaskStatistic, ()> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let statistics = PeriodTaskStatistic::load(&mut connection).await.unwrap();

        Ok(statistics)
    }
}

#[derive(Serialize)]
pub struct PeriodTaskStatistic(HashMap<String, DateTaskStatistic>);

impl PeriodTaskStatistic {
    pub async fn load(connection: &mut PoolConnection<Sqlite>) -> Result<Self, ()> {
        let mut statistics = PeriodTaskStatistic(HashMap::<String, DateTaskStatistic>::new());

        let sqlx_result = sqlx::query(
           "SELECT COUNT(*) as count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc IS NOT NULL GROUP BY date ORDER BY date DESC",
        )
        .fetch_all(&mut **connection)
        .await.unwrap();

        for row in sqlx_result {
            let date = row.get("date");
            let level = match row.get("count") {
                0 => 0,
                1..=3 => 1,
                4..=6 => 2,
                7..=9 => 3,
                _ => 4,
            };
            let date_statistic = DateTaskStatistic {
                level: level,
                data: DateTaskStatisticData {
                    completed_tasks: row.get("count"),
                },
            };

            statistics.0.insert(date, date_statistic);
        }

        Ok(statistics)
    }
}

#[derive(Serialize)]
pub struct DateTaskStatistic {
    level: i64,
    data: DateTaskStatisticData,
}

#[derive(Serialize)]
pub struct DateTaskStatisticData {
    completed_tasks: i64,
}

#[tauri::command]
pub async fn load_task_activity_statistics_command(
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load task activity statistics command");

    let manager = TaskManager::new(&db_pool).unwrap();

    let statistics = manager.load_statistics().await.unwrap();

    Ok(serde_json::to_string(&statistics).unwrap())
}

#[tauri::command]
pub async fn load_tasks_inbox_command(db_pool: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks inbox command");

    let manager = TaskManager::new(&db_pool).unwrap();

    let tasks = manager.load_inbox().await.unwrap();

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_due_today_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks due today command");

    let manager = TaskManager::new(&db).unwrap();

    let tasks = manager.load_due_before(Utc::now()).await.unwrap();

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_with_deadline_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks with deadline command");

    let manager = TaskManager::new(&db).unwrap();

    let tasks = manager.load_with_deadline().await.unwrap();

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn complete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running complete task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| e.to_string())?;

    let manager = TaskManager::new(&db).unwrap();

    manager.complete_task(uuid).await.unwrap();

    Ok("{}".to_string())
}

#[tauri::command]
pub async fn load_tasks_command(
    include_completed: bool,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!(
        "Running load tasks command - include_completed: {:?}",
        include_completed
    );

    let manager = TaskManager::new(&db).unwrap();

    let tasks = manager.load_tasks(include_completed).await.unwrap();

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn delete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running delete task command for card ID: {}", task_id);

    let task_manager = TaskManager::new(&db).unwrap();
    let task_uuid = Uuid::parse_str(&task_id).unwrap();

    let _ = task_manager.delete_task(task_uuid).await.unwrap();

    Ok(format!("Task with ID {} deleted successfully", &task_id))
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

    let uuid: Uuid = Uuid::parse_str(&task_id).unwrap();

    match task_manager.update_task(uuid, updated_task_data).await {
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

#[tauri::command]
pub async fn create_task_command(
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let due_at_utc = match due_date {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).unwrap(),
        )),
        None => None,
    };

    let deadline_at_utc = match deadline {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).unwrap(),
        )),
        None => None,
    };

    let project_id_uuid = match project_id {
        Some(s) => Some(Uuid::parse_str(&s).unwrap()),
        None => None,
    };

    let create_task_data = CreateTaskData {
        title,
        description,
        due_at_utc,
        deadline_at_utc,
        project_id: project_id_uuid,
    };

    log::debug!("Running update task command for: | {:?}", create_task_data);

    let task_manager = TaskManager::new(&db).unwrap();

    match task_manager.create_task(create_task_data).await {
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
    use crate::*;
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

        let create_task_data = CreateTaskData {
            title: "Created Task".to_string(),
            project_id: None,
            description: None,
            due_at_utc: None,
            deadline_at_utc: None,
        };

        let task = manager.create_task(create_task_data).await.unwrap();

        let updated_task_data = UpdatedTaskData {
            title: "Updated task".to_string(),
            description: Some("Updated description".to_string()),
            due_date: None,
            deadline: None,
            project_id: None,
        };

        let updated_task = manager
            .update_task(task.id, updated_task_data)
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
