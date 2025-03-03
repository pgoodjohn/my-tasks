use std::collections::HashMap;
use std::error::Error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite};
use uuid::Uuid;

use crate::project::Project;

pub mod manager;
pub mod tauri;
mod test;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedTaskData {
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskData {
    project_id: Option<Uuid>,
    title: String,
    description: Option<String>,
    due_at_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project: Option<Project>,
    pub parent_task_id: Option<Uuid>,
    pub due_at_utc: Option<DateTime<Utc>>,
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
    ) -> Self {
        Task {
            id: Uuid::now_v7(),
            title,
            description,
            project,
            parent_task_id: None,
            due_at_utc,
            created_at_utc: Utc::now(),
            completed_at_utc: None,
            updated_at_utc: Utc::now(),
        }
    }

    pub async fn update(
        &mut self,
        data: UpdatedTaskData,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<(), Box<dyn Error>> {
        self.title = data.title;
        self.description = data.description;
        self.due_at_utc = data
            .due_date
            .map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
        self.updated_at_utc = Utc::now();

        match data.project_id {
            Some(project_id) => {
                let project_uuid = Uuid::parse_str(&project_id)?;
                self.project = Project::load_by_id(project_uuid, connection).await.unwrap();
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
        let stored_task = Task::load_by_id(self.id, connection).await.unwrap();

        match stored_task {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn update_record(
        &mut self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<&Self, sqlx::Error> {
        self.updated_at_utc = Utc::now();

        let _sql_result = sqlx::query(
            "UPDATE tasks SET title = ?1, description = ?2, due_at_utc = ?3, parent_task_id = ?4, updated_at_utc = ?5, project_id = ?6, completed_at_utc = ?7, updated_at_utc = ?8 WHERE id = ?9"
        )
        .bind(&self.title)
        .bind(&self.description)
        .bind(self.due_at_utc.map(|date| date.to_rfc3339()))
        .bind(self.parent_task_id.map(|task_uuid| task_uuid.to_string()))
        .bind(self.updated_at_utc.to_rfc3339())
        .bind(self.project.as_ref().map(|project| project.id.to_string()))
        .bind(self.completed_at_utc.map(|date| date.to_rfc3339()))
        .bind(self.updated_at_utc.to_rfc3339())
        .bind(self.id.to_string())
        .execute(&mut **connection).await?;

        Ok(self)
    }

    pub async fn create_record(
        &mut self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<&Self, sqlx::Error> {
        if self.is_stored(connection).await? {
            self.update_record(connection).await?;
            return Ok(self);
        }

        let _sql_result = sqlx::query(
            "INSERT INTO tasks (id, title, description, project_id, parent_task_id, due_at_utc, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )
        .bind(self.id.to_string())
        .bind(&self.title)
        .bind(&self.description)
        .bind(self.project.as_ref().map(|project| project.id.to_string()))
        .bind(self.parent_task_id.map(|id| id.to_string()))
        .bind(self.due_at_utc.map(|date| date.to_rfc3339()))
        .bind(self.created_at_utc.to_rfc3339())
        .bind(self.updated_at_utc.to_rfc3339())
        .execute(&mut **connection).await?;

        Ok(self)
    }

    pub async fn delete_record(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        let _sql_result = sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(self.id.to_string())
            .execute(&mut **connection)
            .await?;

        Ok(())
    }

    async fn from_sqlx_row(
        row: sqlx::sqlite::SqliteRow,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, Box<dyn Error>> {
        let uuid_string: String = row.get("id");
        let project_uuid_string: Option<String> = row.get("project_id");
        let parent_task_uuid_string: Option<String> = row.get("parent_task_id");
        let due_at_utc_string: Option<String> = row.get("due_at_utc");
        let created_at_string: String = row.get("created_at_utc");
        let updated_at_string: String = row.get("updated_at_utc");
        let completed_at_string: Option<String> = row.get("completed_at_utc");

        Ok(Task {
            id: Uuid::parse_str(&uuid_string)?,
            title: row.get("title"),
            description: row.get("description"),
            project: match project_uuid_string {
                Some(uuid) => {
                    Project::load_by_id(Uuid::parse_str(&uuid).unwrap(), &mut *connection)
                        .await
                        .unwrap()
                }
                None => None,
            },
            parent_task_id: parent_task_uuid_string.map(|s| Uuid::parse_str(&s).unwrap()),
            due_at_utc: due_at_utc_string
                .map(|s| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&s).unwrap())),
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
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let rows = sqlx::query("SELECT * FROM tasks WHERE id = ?1 LIMIT 1")
            .bind(id.to_string())
            .fetch_all(&mut **connection)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks.pop())
    }

    pub async fn load_filtered_by_completed(
        include_completed: bool,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let query = match include_completed {
            true => "SELECT * FROM tasks ORDER BY updated_at_utc DESC",
            false => {
                "SELECT * FROM tasks WHERE completed_at_utc IS NULL ORDER BY updated_at_utc DESC"
            }
        };

        let rows = sqlx::query(query).fetch_all(&mut **connection).await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_completed_tasks(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE completed_at_utc IS NOT NULL ORDER BY completed_at_utc DESC",
        )
        .fetch_all(&mut **connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_for_project(
        project_id: Uuid,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows =
            sqlx::query("SELECT * FROM tasks WHERE project_id = ?1 ORDER BY updated_at_utc DESC")
                .bind(project_id.to_string())
                .fetch_all(&mut **connection)
                .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_for_parent(
        parent_task_id: Uuid,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE parent_task_id = ?1 ORDER BY updated_at_utc DESC",
        )
        .bind(parent_task_id.to_string())
        .fetch_all(&mut **connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_due_before(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE due_at_utc < ?1 AND completed_at_utc IS NULL ORDER BY due_at_utc ASC"
        )
        .bind(date.to_rfc3339())
        .fetch_all(&mut **connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }

    pub async fn load_inbox(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE project_id IS NULL AND completed_at_utc IS NULL ORDER BY created_at_utc DESC"
        )
        .fetch_all(&mut **connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task::from_sqlx_row(row, connection).await?;
            tasks.push(task);
        }

        Ok(tasks)
    }
}

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
