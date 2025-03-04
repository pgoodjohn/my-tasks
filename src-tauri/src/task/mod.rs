use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

pub mod manager;
pub mod repository;
pub mod tauri;
mod test;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatedTaskData {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskData {
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub due_at_utc: Option<String>,
}

#[derive(Debug, FromRow)]
struct TaskRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub due_at_utc: Option<String>,
    pub created_at_utc: String,
    pub completed_at_utc: Option<String>,
    pub updated_at_utc: String,
}

impl TryFrom<TaskRow> for Task {
    type Error = Box<dyn Error>;

    fn try_from(row: TaskRow) -> Result<Self, Self::Error> {
        Ok(Task {
            id: Uuid::parse_str(&row.id)?,
            title: row.title,
            description: row.description,
            project_id: row.project_id.map(|id| Uuid::parse_str(&id)).transpose()?,
            parent_task_id: row
                .parent_task_id
                .map(|id| Uuid::parse_str(&id))
                .transpose()?,
            due_at_utc: row
                .due_at_utc
                .map(|date| DateTime::parse_from_rfc3339(&date))
                .transpose()?
                .map(DateTime::<Utc>::from),
            created_at_utc: DateTime::parse_from_rfc3339(&row.created_at_utc)?.with_timezone(&Utc),
            completed_at_utc: row
                .completed_at_utc
                .map(|date| DateTime::parse_from_rfc3339(&date))
                .transpose()?
                .map(DateTime::<Utc>::from),
            updated_at_utc: DateTime::parse_from_rfc3339(&row.updated_at_utc)?.with_timezone(&Utc),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<Uuid>,
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
        project_id: Option<Uuid>,
        parent_task_id: Option<Uuid>,
        due_at_utc: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            id: Uuid::now_v7(),
            title,
            description,
            project_id,
            parent_task_id,
            due_at_utc,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            completed_at_utc: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DateTaskStatistic {
    pub completed_tasks: i64,
    pub created_tasks: i64,
}

#[derive(Debug, Serialize)]
pub struct PeriodTaskStatistic(HashMap<String, DateTaskStatistic>);

impl PeriodTaskStatistic {
    pub async fn load(db_pool: &SqlitePool) -> Result<Vec<Self>, Box<dyn Error>> {
        let completed_tasks = sqlx::query(
            r#"
            SELECT date(completed_at_utc) as date, COUNT(*) as count
            FROM tasks
            WHERE completed_at_utc IS NOT NULL
            GROUP BY date(completed_at_utc)
            ORDER BY date(completed_at_utc) DESC
            "#,
        )
        .fetch_all(db_pool)
        .await?;

        let created_tasks = sqlx::query(
            r#"
            SELECT date(created_at_utc) as date, COUNT(*) as count
            FROM tasks
            GROUP BY date(created_at_utc)
            ORDER BY date(created_at_utc) DESC
            "#,
        )
        .fetch_all(db_pool)
        .await?;

        let mut period_statistic = PeriodTaskStatistic(HashMap::new());

        for row in completed_tasks {
            let date: String = row.get("date");
            let count: i64 = row.get("count");

            period_statistic
                .0
                .entry(date)
                .or_insert(DateTaskStatistic {
                    completed_tasks: 0,
                    created_tasks: 0,
                })
                .completed_tasks = count;
        }

        for row in created_tasks {
            let date: String = row.get("date");
            let count: i64 = row.get("count");

            period_statistic
                .0
                .entry(date)
                .or_insert(DateTaskStatistic {
                    completed_tasks: 0,
                    created_tasks: 0,
                })
                .created_tasks = count;
        }

        Ok(vec![period_statistic])
    }
}
