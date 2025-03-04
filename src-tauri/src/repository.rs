use sqlx::{Pool, Sqlite};

use crate::project::repository::SqliteProjectRepository;
use crate::task::repository::SqliteTaskRepository;

pub struct RepositoryProvider {
    pub pool: Pool<Sqlite>,
}

impl RepositoryProvider {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn task_repository(&self) -> Result<SqliteTaskRepository, sqlx::Error> {
        let connection = self.pool.acquire().await?;
        Ok(SqliteTaskRepository::new(connection))
    }

    pub async fn project_repository(&self) -> Result<SqliteProjectRepository, sqlx::Error> {
        let connection = self.pool.acquire().await?;
        Ok(SqliteProjectRepository::new(connection))
    }
}
