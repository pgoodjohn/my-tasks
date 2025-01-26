use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite};
use uuid::Uuid;

use crate::task::Task;

pub mod commands;
mod test;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub emoji: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,
    pub archived_at_utc: Option<DateTime<Utc>>,
    #[serde(rename(serialize = "isFavorite"))]
    pub is_favorite: bool,
}

#[derive(Debug, Serialize)]
pub struct ProjectDetail {
    pub project: Project,
    pub tasks: Vec<Task>,
}

impl Project {
    pub fn new(
        title: String,
        emoji: Option<String>,
        color: Option<String>,
        description: Option<String>,
    ) -> Self {
        Project {
            id: Uuid::now_v7(),
            title: title,
            emoji: emoji,
            color: color,
            description: description,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            archived_at_utc: None,
            is_favorite: false,
        }
    }

    pub async fn save(&mut self, connection: &mut PoolConnection<Sqlite>) -> Result<&Self, ()> {
        if self.exists(connection).await.unwrap() {
            return self.update_record(connection).await;
        }

        let _sql_result = sqlx::query(
            "INSERT INTO projects (id, title, color, emoji, description, created_at_utc, updated_at_utc, is_favorite) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)")
            .bind(&self.id.to_string())
            .bind(&self.title)
            .bind(&self.color)
            .bind(&self.emoji)
            .bind(&self.description)
            .bind(&self.created_at_utc.to_rfc3339())
            .bind(&self.updated_at_utc.to_rfc3339())
            .execute(&mut **connection).await.unwrap();

        Ok(self)
    }

    async fn exists(&self, connection: &mut PoolConnection<Sqlite>) -> Result<bool, String> {
        let stored_task = Project::load_by_id(self.id, connection).await.unwrap();

        match stored_task {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn update_record(&self, connection: &mut PoolConnection<Sqlite>) -> Result<&Self, ()> {
        let _sql_result = sqlx::query(
            "UPDATE projects SET title = ?1, emoji = ?2, color = ?3, description = ?4, updated_at_utc = ?5, archived_at_utc = ?6, is_favorite =?7 WHERE id = ?8")
            .bind(&self.title)
            .bind(&self.emoji)
            .bind(&self.color)
            .bind(&self.description)
            .bind(&self.updated_at_utc.to_rfc3339())
            .bind(self.archived_at_utc.map(|date| date.to_rfc3339()))
            .bind(self.is_favorite)
            .bind(&self.id.to_string())
            .execute(&mut **connection).await.unwrap();

        Ok(self)
    }
    pub async fn load_by_id(
        id: Uuid,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Option<Self>, ()> {
        let mut rows = sqlx::query("SELECT * FROM projects WHERE id = ?1 LIMIT 1")
            .bind(id.to_string())
            .fetch_all(&mut **connection)
            .await
            .unwrap()
            .into_iter()
            .map(|row: sqlx::sqlite::SqliteRow| {
                Project::from_sqlx_row(row).unwrap() // TODO: unwrap
            })
            .collect::<Vec<_>>();

        return Ok(rows.pop());
    }

    pub async fn list_not_archived_projects(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Project>, ()> {
        let rows = sqlx::query("SELECT * FROM projects WHERE archived_at_utc IS NULL")
            .fetch_all(&mut **connection)
            .await
            .unwrap();

        let mut projects = Vec::new();
        for row in rows {
            let project = Project::from_sqlx_row(row).unwrap(); // TODO: unwrap
            projects.push(project);
        }

        return Ok(projects);
    }

    pub async fn list_all_projects(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Project>, ()> {
        let rows = sqlx::query("SELECT * FROM projects")
            .fetch_all(&mut **connection)
            .await
            .unwrap();

        let mut projects = Vec::new();
        for row in rows {
            let project = Project::from_sqlx_row(row).unwrap(); // TODO: unwrap
            projects.push(project);
        }

        return Ok(projects);
    }

    pub async fn list_favorite_projects(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Project>, ()> {
        let rows = sqlx::query("SELECT * FROM projects WHERE is_favorite = true")
            .fetch_all(&mut **connection)
            .await
            .unwrap();

        let mut projects = Vec::new();
        for row in rows {
            let project = Project::from_sqlx_row(row).unwrap(); // TODO: unwrap
            projects.push(project);
        }

        return Ok(projects);
    }

    fn from_sqlx_row(row: sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let uuid_string: String = row.get("id");
        let created_at_string: String = row.get("created_at_utc");
        let updated_at_string: String = row.get("updated_at_utc");
        let archived_at_string: Option<String> = row.get("archived_at_utc");

        Ok(Project {
            id: Uuid::parse_str(&uuid_string).unwrap(),
            title: row.get("title"),
            emoji: row.get("emoji"),
            color: row.get("color"),
            description: row.get("description"),
            created_at_utc: DateTime::<Utc>::from(
                DateTime::parse_from_rfc3339(&created_at_string).unwrap(),
            ),
            updated_at_utc: DateTime::<Utc>::from(
                DateTime::parse_from_rfc3339(&updated_at_string).unwrap(),
            ),
            archived_at_utc: match archived_at_string {
                Some(s) => Some(DateTime::<Utc>::from(
                    DateTime::parse_from_rfc3339(&s).unwrap(),
                )),
                None => None,
            },
            is_favorite: row.get("is_favorite"),
        })
    }

    pub async fn count_open_tasks_for_project(
        &self,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<i64, ()> {
        let result = sqlx::query(
            "SELECT COUNT(*) as `count` FROM tasks WHERE project_id = ?1 AND completed_at_utc IS NULL"
        ).bind(self.id.to_string())
        .fetch_all(&mut **connection)
        .await
        .unwrap();

        let mut count: i64 = 0;
        for row in result {
            let count_row: sqlx::sqlite::SqliteRow = row; // TODO: unwrap
            let count_value: i64 = count_row.get("count");
            count += count_value;
        }

        Ok(count)
    }
}
