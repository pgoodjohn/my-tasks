use sqlx::{Pool, Sqlite};

use crate::project::repository::{ProjectRepository, SqliteProjectRepository};
use crate::recurring_task::repository::{RecurringTaskRepository, SqliteRecurringTaskRepository};
use crate::task::repository::{SqliteTaskRepository, TaskRepository};

pub struct RepositoryProvider {
    pub pool: Pool<Sqlite>,
}

impl RepositoryProvider {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn task_repository(&self) -> Result<impl TaskRepository, sqlx::Error> {
        let connection = self.pool.acquire().await?;
        Ok(SqliteTaskRepository::new(connection))
    }

    pub async fn project_repository(&self) -> Result<impl ProjectRepository, sqlx::Error> {
        let connection = self.pool.acquire().await?;
        Ok(SqliteProjectRepository::new(connection))
    }

    pub async fn recurring_task_repository(
        &self,
    ) -> Result<impl RecurringTaskRepository, sqlx::Error> {
        let connection = self.pool.acquire().await?;
        Ok(SqliteRecurringTaskRepository::new(connection))
    }
}
