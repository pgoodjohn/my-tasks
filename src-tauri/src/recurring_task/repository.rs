use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{pool::PoolConnection, Sqlite};
use uuid::Uuid;

use super::RecurringTask;

#[async_trait]
pub trait RecurringTaskRepository: Send + Sync {
    async fn save(&mut self, recurring_task: &mut RecurringTask) -> Result<(), sqlx::Error>;
    async fn find_by_task_id(
        &mut self,
        task_id: Uuid,
    ) -> Result<Option<RecurringTask>, sqlx::Error>;
    async fn find_due_before(
        &mut self,
        date: DateTime<Utc>,
    ) -> Result<Vec<RecurringTask>, sqlx::Error>;
    async fn delete(&mut self, recurring_task: &RecurringTask) -> Result<(), sqlx::Error>;
}

pub struct SqliteRecurringTaskRepository {
    connection: PoolConnection<Sqlite>,
}

impl SqliteRecurringTaskRepository {
    pub fn new(connection: PoolConnection<Sqlite>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl RecurringTaskRepository for SqliteRecurringTaskRepository {
    async fn save(&mut self, recurring_task: &mut RecurringTask) -> Result<(), sqlx::Error> {
        let exists = sqlx::query("SELECT 1 FROM recurring_tasks WHERE id = ?1 LIMIT 1")
            .bind(recurring_task.id.to_string())
            .fetch_optional(&mut *self.connection)
            .await?
            .is_some();

        recurring_task.updated_at_utc = Utc::now();

        if exists {
            sqlx::query(
                "UPDATE recurring_tasks SET task_id = ?1, frequency = ?2, interval = ?3, next_due_at_utc = ?4, updated_at_utc = ?5 WHERE id = ?6"
            )
            .bind(recurring_task.task_id.to_string())
            .bind(&recurring_task.frequency)
            .bind(recurring_task.interval)
            .bind(recurring_task.next_due_at_utc.to_rfc3339())
            .bind(recurring_task.updated_at_utc.to_rfc3339())
            .bind(recurring_task.id.to_string())
            .execute(&mut *self.connection)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO recurring_tasks (id, task_id, frequency, interval, next_due_at_utc, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )
            .bind(recurring_task.id.to_string())
            .bind(recurring_task.task_id.to_string())
            .bind(&recurring_task.frequency)
            .bind(recurring_task.interval)
            .bind(recurring_task.next_due_at_utc.to_rfc3339())
            .bind(recurring_task.created_at_utc.to_rfc3339())
            .bind(recurring_task.updated_at_utc.to_rfc3339())
            .execute(&mut *self.connection)
            .await?;
        }

        Ok(())
    }

    async fn find_by_task_id(
        &mut self,
        task_id: Uuid,
    ) -> Result<Option<RecurringTask>, sqlx::Error> {
        sqlx::query_as::<_, RecurringTask>(
            "SELECT * FROM recurring_tasks WHERE task_id = ?1 LIMIT 1",
        )
        .bind(task_id.to_string())
        .fetch_optional(&mut *self.connection)
        .await
    }

    async fn find_due_before(
        &mut self,
        date: DateTime<Utc>,
    ) -> Result<Vec<RecurringTask>, sqlx::Error> {
        sqlx::query_as::<_, RecurringTask>(
            "SELECT * FROM recurring_tasks WHERE next_due_at_utc < ?1 ORDER BY next_due_at_utc ASC",
        )
        .bind(date.to_rfc3339())
        .fetch_all(&mut *self.connection)
        .await
    }

    async fn delete(&mut self, recurring_task: &RecurringTask) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM recurring_tasks WHERE id = ?1")
            .bind(recurring_task.id.to_string())
            .execute(&mut *self.connection)
            .await?;

        Ok(())
    }
}
