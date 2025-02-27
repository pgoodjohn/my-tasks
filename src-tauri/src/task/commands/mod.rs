use std::collections::HashMap;

use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite, SqlitePool};
use tauri::State;
use uuid::Uuid;

use super::{CreateTaskData, UpdatedTaskData};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::error::Error;

use crate::errors::handle_error;

use super::manager::TaskManager;

#[derive(Serialize)]
pub struct PeriodTaskStatistic(HashMap<String, DateTaskStatistic>);

impl PeriodTaskStatistic {
    pub async fn load(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut statistics = vec![];

        let sqlx_result = sqlx::query(
           "SELECT COUNT(*) as count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc IS NOT NULL GROUP BY date ORDER BY date DESC",
        )
        .fetch_all(&mut **connection)
        .await?;

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
                level,
                data: DateTaskStatisticData {
                    completed_tasks: row.get("count"),
                },
            };

            let mut period_statistic =
                PeriodTaskStatistic(HashMap::<String, DateTaskStatistic>::new());
            period_statistic.0.insert(date, date_statistic);

            statistics.push(period_statistic)
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

    let manager = TaskManager::new(&db_pool);

    let statistics = manager
        .load_statistics()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&statistics).unwrap())
}

#[tauri::command]
pub async fn load_tasks_inbox_command(db_pool: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks inbox command");

    let manager = TaskManager::new(&db_pool);

    let tasks = manager.load_inbox().await.map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_due_today_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks due today command");

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_due_before(Utc::now())
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn load_tasks_with_deadline_command(db: State<'_, SqlitePool>) -> Result<String, String> {
    log::debug!("Running load tasks with deadline command");

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_with_deadline()
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn complete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running complete task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let manager = TaskManager::new(&db);

    manager
        .complete_task(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

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

    let manager = TaskManager::new(&db);

    let tasks = manager
        .load_tasks(include_completed)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub async fn delete_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running delete task command for card ID: {}", task_id);

    let task_manager = TaskManager::new(&db);
    let task_uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    task_manager
        .delete_task(task_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

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

    let task_manager = TaskManager::new(&db);

    let uuid: Uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let task = task_manager
        .update_task(uuid, updated_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
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
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let deadline_at_utc = match deadline {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let project_id_uuid = match project_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|e| handle_error(&e))?),
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

    let task_manager = TaskManager::new(&db);

    let task = task_manager
        .create_task(create_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn create_subtask_for_task_command(
    parent_task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let due_at_utc = match due_date {
        Some(date) => Some(DateTime::<Utc>::from(
            DateTime::parse_from_rfc3339(&date).map_err(|e| handle_error(&e))?,
        )),
        None => None,
    };

    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;

    let task_manager = TaskManager::new(&db);

    let parent_task = task_manager
        .load_by_id(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?
        .unwrap(); // TODO: Remove this unwrap is parent task cant be found anymore (shouldnt happen really)

    let create_task_data = CreateTaskData {
        title,
        project_id: parent_task.project.as_ref().map(|p| p.id),
        description,
        due_at_utc,
        deadline_at_utc: parent_task.deadline_at_utc,
    };

    let subtask = task_manager
        .create_subtask_for_task(parent_task, create_task_data)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtask).unwrap())
}

#[tauri::command]
pub async fn load_subtasks_for_task_command(
    parent_task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let parent_task_id_uuid = Uuid::parse_str(&parent_task_id).map_err(|e| handle_error(&e))?;
    let task_manager = TaskManager::new(&db);

    let subtasks = task_manager
        .load_subtasks_for_task(parent_task_id_uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&subtasks).unwrap())
}

#[tauri::command]
pub async fn load_task_by_id_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;
    let manager = TaskManager::new(&db);

    let task = manager
        .load_by_id(uuid)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub async fn tick_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running tick task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| handle_error(&e))?;

    let manager = TaskManager::new(&db);

    manager.tick(uuid).await.map_err(|e| handle_error(&*e))?;

    Ok("{}".to_string())
}
