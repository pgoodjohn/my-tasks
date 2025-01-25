use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite};
use tauri::State;
use tokio::task;
use uuid::Uuid;

use crate::project::Project;

pub mod commands;
mod test;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedTaskData {
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    deadline: Option<String>,
    project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskData {
    project_id: Option<Uuid>,
    title: String,
    description: Option<String>,
    due_at_utc: Option<DateTime<Utc>>,
    deadline_at_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project: Option<Project>,
    pub due_at_utc: Option<DateTime<Utc>>,
    pub deadline_at_utc: Option<DateTime<Utc>>,
    pub created_at_utc: DateTime<Utc>,
    pub completed_at_utc: Option<DateTime<Utc>>,
    pub updated_at_utc: DateTime<Utc>,
}

impl Task {
    pub fn new(
        title: String,
        description: Option<String>,
        project: Option<Project>,
        due_at_utc: Option<DateTime<Utc>>,
        deadline_at_utc: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            id: Uuid::now_v7(),
            title: title,
            description: description,
            project: project,
            due_at_utc: due_at_utc,
            deadline_at_utc: deadline_at_utc,
            created_at_utc: Utc::now(),
            completed_at_utc: None,
            updated_at_utc: Utc::now(),
        }
    }

    pub async fn update(
        &mut self,
        data: UpdatedTaskData,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<(), commands::TaskError> {
        self.title = data.title;
        self.description = data.description;
        self.due_at_utc = data
            .due_date
            .map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
        self.deadline_at_utc = data
            .deadline
            .map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
        self.updated_at_utc = Utc::now();

        match data.project_id {
            Some(project_id) => {
                let project_uuid = Uuid::parse_str(&project_id)?;
                self.project = Project::load_by_id(project_uuid, connection).await?;
            }
            None => {
                self.project = None;
            }
        }
        Ok(())
    }

    async fn is_stored(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<bool, sqlx::Error> {
        let stored_task = Task::load_by_id(self.id, connection).await?;

        match stored_task {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn update_record(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<&Self, sqlx::Error> {
        let _sql_result = sqlx::query(
            "UPDATE tasks SET title = ?1, description = ?2, due_at_utc = ?3, updated_at_utc = ?4, project_id = ?5, deadline_at_utc = ?6 WHERE id = ?7"
        )
        .bind(&self.title)
        .bind(&self.description)
        .bind(self.due_at_utc.map(|date| date.to_rfc3339()))
        .bind(&self.updated_at_utc.to_rfc3339())
        .bind(self.project.as_ref().map(|project| project.id.to_string()))
        .bind(self.deadline_at_utc.map(|date| date.to_rfc3339()))
        .bind(&self.id.to_string())
        .execute(&mut **connection).await?;

        Ok(self)
    }

    pub async fn create_record(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<&Self, sqlx::Error> {
        if self.is_stored(connection).await? {
            self.update_record(connection).await?;
            return Ok(self);
        }

        let _sql_result = sqlx::query(
            "INSERT INTO tasks (id, title, description, project_id, due_at_utc, deadline_at_utc, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )
        .bind(&self.id.to_string())
        .bind(&self.title)
        .bind(&self.description)
        .bind(self.project.as_ref().map(|project| project.id.to_string()))
        .bind(self.due_at_utc.map(|date| date.to_rfc3339()))
        .bind(self.deadline_at_utc.map(|date| date.to_rfc3339()))
        .bind(&self.created_at_utc.to_rfc3339())
        .bind(&self.updated_at_utc.to_rfc3339())
        .execute(&mut **connection).await?;

        Ok(self)
    }

    pub async fn delete_record(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let _sql_result = sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(&self.id.to_string())
            .execute(&mut **connection)
            .await?;

        Ok(())
    }

    async fn from_sqlx_row(
        row: sqlx::sqlite::SqliteRow,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let uuid_string: String = row.get("id");
        let project_uuid_string: Option<String> = row.get("project_id");
        let created_at_string: String = row.get("created_at_utc");
        let updated_at_string: String = row.get("updated_at_utc");
        let completed_at_string: Option<String> = row.get("completed_at_utc");

        Ok(Task {
            id: Uuid::parse_str(&uuid_string).unwrap(),
            title: row.get("title"),
            description: row.get("description"),
            project: match project_uuid_string {
                Some(uuid) => Project::load_by_id(Uuid::parse_str(&uuid).unwrap(), &mut connection)
                    .await
                    .unwrap(),
                None => None,
            },
            due_at_utc: row.get("due_at_utc").map(|date: String| {
                DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap())
            }),
            deadline_at_utc: row.get("deadline_at_utc").map(|date: String| {
                DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap())
            }),
            created_at_utc: DateTime::<Utc>::from(
                DateTime::parse_from_rfc3339(&created_at_string).unwrap(),
            ),
            completed_at_utc: completed_at_string
                .map(|s| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&s).unwrap())),
            updated_at_utc: DateTime::<Utc>::from(
                DateTime::parse_from_rfc3339(&updated_at_string).unwrap(),
            ),
        })
    }

    pub async fn load_by_id(
        id: Uuid,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<Self>> {
        let rows = sqlx::query("SELECT * FROM tasks WHERE id = ?1 LIMIT 1")
            .bind(id.to_string())
            .fetch_all(&mut **connection)
            .await
            .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap(); // TODO: unwrap
            tasks.push(task);
        }

        return Ok(tasks.pop());
    }

    pub async fn load_filtered_by_completed(
        include_completed: bool,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>> {
        let query = match include_completed {
            true => "SELECT * FROM tasks ORDER BY created_at_utc DESC",
            false => {
                "SELECT * FROM tasks WHERE completed_at_utc IS NULL ORDER BY created_at_utc DESC"
            }
        };

        let rows = sqlx::query(query)
            .fetch_all(&mut **connection)
            .await
            .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap(); // TODO: unwrap
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_for_project(
        project_id: Uuid,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>> {
        let rows =
            sqlx::query("SELECT * FROM tasks WHERE project_id = ?1 ORDER BY created_at_utc DESC")
                .bind(project_id.to_string())
                .fetch_all(&mut **connection)
                .await
                .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap(); // TODO: unwrap
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_due_before(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE due_at_utc < ?1 AND completed_at_utc IS NULL ORDER BY due_at_utc ASC"
        )
        .bind(date.to_rfc3339())
        .fetch_all(&mut **connection)
        .await
        .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap();
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_with_deadlines(connection: &mut PoolConnection<Sqlite>) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE deadline_at_utc IS NOT NULL AND completed_at_utc IS NULL ORDER BY deadline_at_utc ASC"
        )
        .fetch_all(&mut **connection)
        .await
        .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap();
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_inbox(connection: &mut PoolConnection<Sqlite>) -> Result<Vec<Self>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE project_id IS NULL AND completed_at_utc IS NULL ORDER BY created_at_utc DESC"
        )
        .fetch_all(&mut **connection)
        .await
        .unwrap();

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await.unwrap();
            tasks.push(task);
        }

        Ok(tasks)
    }
}

#[tauri::command]
pub fn count_open_tasks_for_project_command(
    project_id: String,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!(
        "Running count open tasks for project command for project ID: {}",
        project_id
    );
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    let project = Project::load_by_id(uuid, &conn).unwrap();

    match project {
        Some(project) => {
            let count = project.count_open_tasks_for_project(&conn).unwrap();
            return Ok(count.to_string());
        }
        None => {
            return Err("Project not found".to_string());
        }
    }
}

#[tauri::command]
pub fn load_task_activity_statistics_command(
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running load task activity statistics command");
    let conn = db.get().unwrap(); // Get a connection from the pool
    let mut stmt = conn.prepare("SELECT COUNT(*) as count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc IS NOT NULL GROUP BY date ORDER BY date DESC").unwrap();
    let task_iter = stmt
        .query_map([], |row| Ok((row.get("date")?, row.get("count")?)))
        .unwrap();

    let mut statistics = Vec::new();
    for task in task_iter {
        let (date, count): (String, i64) = task.unwrap();
        let level = match count {
            0 => 0,
            1..=3 => 1,
            4..=6 => 2,
            7..=9 => 3,
            _ => 4,
        };
        let mut entry = serde_json::Map::new();
        entry.insert("level".to_string(), serde_json::json!(level));
        entry.insert(
            "data".to_string(),
            serde_json::json!({ "completedTasks": count }),
        );
        statistics.push(serde_json::json!({ date: entry }));
    }

    Ok(serde_json::to_string(&statistics).unwrap())
}
