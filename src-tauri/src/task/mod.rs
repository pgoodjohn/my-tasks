use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

use crate::project::Project;

pub mod manager;
pub mod repository;
pub mod tauri;
mod test;

use repository::{RepositoryProvider, TaskRepository};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project_id: Option<Uuid>,
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
        project_id: Option<Uuid>,
        parent_task_id: Option<Uuid>,
        due_at_utc: Option<DateTime<Utc>>,
    ) -> Self {
        Task {
            id: Uuid::now_v7(),
            title,
            description,
            project_id,
            project: None,
            parent_task_id,
            due_at_utc,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            completed_at_utc: None,
        }
    }

    pub fn from_create_data(data: CreateTaskData) -> Result<Self, Box<dyn Error>> {
        Ok(Task {
            id: Uuid::now_v7(),
            title: data.title,
            description: data.description,
            project_id: match data.project_id {
                Some(id) => Some(Uuid::parse_str(&id)?),
                None => None,
            },
            project: None,
            parent_task_id: None,
            due_at_utc: data
                .due_at_utc
                .map(|date| DateTime::parse_from_rfc3339(&date))
                .transpose()?
                .map(DateTime::<Utc>::from),
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            completed_at_utc: None,
        })
    }

    pub async fn update(
        &mut self,
        data: UpdatedTaskData,
        db_pool: &SqlitePool,
    ) -> Result<(), Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;

        self.title = data.title;
        self.description = data.description;
        self.due_at_utc = data
            .due_date
            .map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
        self.updated_at_utc = Utc::now();

        match data.project_id {
            Some(project_id) => {
                let project_uuid = Uuid::parse_str(&project_id)?;
                self.project_id = Some(project_uuid);
                self.project = None; // Will be loaded by repository when needed
            }
            None => {
                self.project_id = None;
                self.project = None;
            }
        }

        repository.save(self).await?;
        Ok(())
    }

    pub async fn create_record(&mut self, db_pool: &SqlitePool) -> Result<&Self, sqlx::Error> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        repository.save(self).await?;
        Ok(self)
    }

    pub async fn delete_record(&self, db_pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        repository.delete(self).await
    }

    pub async fn load_by_id(
        id: Uuid,
        db_pool: &SqlitePool,
    ) -> Result<Option<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_by_id(id).await?)
    }

    pub async fn load_filtered_by_completed(
        include_completed: bool,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository
            .find_all_filtered_by_completed(include_completed)
            .await?)
    }

    pub async fn load_completed_tasks(db_pool: &SqlitePool) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_completed().await?)
    }

    pub async fn load_for_project(
        project_id: Uuid,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_by_project(project_id).await?)
    }

    pub async fn load_for_parent(
        parent_task_id: Uuid,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_by_parent(parent_task_id).await?)
    }

    pub async fn load_completed_for_parent(
        parent_task_id: Uuid,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_completed_by_parent(parent_task_id).await?)
    }

    pub async fn load_due_before(
        date: DateTime<Utc>,
        db_pool: &SqlitePool,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_due_before(date).await?)
    }

    pub async fn load_inbox(db_pool: &SqlitePool) -> Result<Vec<Self>, Box<dyn Error>> {
        let repository_provider = RepositoryProvider::new(db_pool.clone());
        let mut repository = repository_provider.task_repository().await?;
        Ok(repository.find_inbox().await?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateTaskStatistic {
    pub date: String,
    pub completed_tasks: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeriodTaskStatistic(HashMap<String, DateTaskStatistic>);

impl PeriodTaskStatistic {
    pub async fn load(db_pool: &SqlitePool) -> Result<Vec<Self>, Box<dyn Error>> {
        let rows = sqlx::query(
            r#"
            SELECT
                strftime('%Y-%m-%d', completed_at_utc) as date,
                COUNT(*) as count
            FROM tasks
            WHERE completed_at_utc IS NOT NULL
            GROUP BY strftime('%Y-%m-%d', completed_at_utc)
            ORDER BY date DESC
            "#,
        )
        .fetch_all(db_pool)
        .await?;

        let mut period_statistics = Vec::new();

        for row in rows {
            let date: String = row.get("date");
            let count: i64 = row.get("count");
            let level = match count {
                0..=4 => 1,
                5..=9 => 2,
                _ => 3,
            };
            let date_statistic = DateTaskStatistic {
                date: date.clone(),
                completed_tasks: level as i32,
            };

            let mut period_statistic = PeriodTaskStatistic(HashMap::new());
            period_statistic.0.insert(date, date_statistic);

            period_statistics.push(period_statistic);
        }

        Ok(period_statistics)
    }
}
