use async_trait::async_trait;
use sqlx::{pool::PoolConnection, Row, Sqlite};
use uuid::Uuid;

use super::Project;

#[async_trait]
pub trait ProjectRepository {
    async fn save(&mut self, project: &mut Project) -> Result<(), sqlx::Error>;
    async fn find_by_id(&mut self, id: Uuid) -> Result<Option<Project>, sqlx::Error>;
    async fn find_not_archived(&mut self) -> Result<Vec<Project>, sqlx::Error>;
    async fn find_all(&mut self) -> Result<Vec<Project>, sqlx::Error>;
    async fn find_favorites(&mut self) -> Result<Vec<Project>, sqlx::Error>;
    async fn count_open_tasks(&mut self, project_id: Uuid) -> Result<i64, sqlx::Error>;
}

pub struct SqliteProjectRepository {
    connection: PoolConnection<Sqlite>,
}

impl SqliteProjectRepository {
    pub fn new(connection: PoolConnection<Sqlite>) -> Self {
        Self { connection }
    }

    fn connection(&mut self) -> &mut PoolConnection<Sqlite> {
        &mut self.connection
    }
}

#[async_trait]
impl ProjectRepository for SqliteProjectRepository {
    async fn save(&mut self, project: &mut Project) -> Result<(), sqlx::Error> {
        let exists = sqlx::query("SELECT 1 FROM projects WHERE id = ?1 LIMIT 1")
            .bind(project.id.to_string())
            .fetch_optional(&mut *self.connection)
            .await?
            .is_some();

        if exists {
            sqlx::query(
                "UPDATE projects SET title = ?1, emoji = ?2, color = ?3, description = ?4, updated_at_utc = ?5, archived_at_utc = ?6, is_favorite = ?7 WHERE id = ?8"
            )
            .bind(&project.title)
            .bind(&project.emoji)
            .bind(&project.color)
            .bind(&project.description)
            .bind(project.updated_at_utc.to_rfc3339())
            .bind(project.archived_at_utc.map(|date| date.to_rfc3339()))
            .bind(project.is_favorite)
            .bind(project.id.to_string())
            .execute(&mut *self.connection)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO projects (id, title, color, emoji, description, created_at_utc, updated_at_utc, is_favorite) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )
            .bind(project.id.to_string())
            .bind(&project.title)
            .bind(&project.color)
            .bind(&project.emoji)
            .bind(&project.description)
            .bind(project.created_at_utc.to_rfc3339())
            .bind(project.updated_at_utc.to_rfc3339())
            .bind(project.is_favorite)
            .execute(&mut *self.connection)
            .await?;
        }

        Ok(())
    }

    async fn find_by_id(&mut self, id: Uuid) -> Result<Option<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?1 LIMIT 1")
            .bind(id.to_string())
            .fetch_optional(&mut *self.connection)
            .await
    }

    async fn find_not_archived(&mut self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE archived_at_utc IS NULL")
            .fetch_all(&mut *self.connection)
            .await
    }

    async fn find_all(&mut self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects")
            .fetch_all(&mut *self.connection)
            .await
    }

    async fn find_favorites(&mut self) -> Result<Vec<Project>, sqlx::Error> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE is_favorite = true")
            .fetch_all(&mut *self.connection)
            .await
    }

    async fn count_open_tasks(&mut self, project_id: Uuid) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM tasks WHERE project_id = ?1 AND completed_at_utc IS NULL"
        )
        .bind(project_id.to_string())
        .fetch_one(&mut *self.connection)
        .await?;

        Ok(result.get("count"))
    }
}
