use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

use super::{CreateTaskData, UpdatedTaskData};
use crate::project::Project;
use crate::task::Task;
use chrono::{DateTime, Utc};
use std::error::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Invalid UUID: {0}")]
    InvalidUUID(#[from] uuid::Error),

    #[error("SQLx error: {0}")]
    SQLxError(#[from] sqlx::Error),

    #[error("Project not found")]
    ProjectNotFound,

    #[error("Task not found")]
    TaskNotFound,
}

pub struct TaskManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> TaskManager<'a> {
    pub fn new(db_pool: &'a SqlitePool) -> Self {
        TaskManager { db_pool }
    }

    pub async fn create_task(
        &self,
        create_task_data: CreateTaskData,
    ) -> Result<Task, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let project = match create_task_data.project_id {
            Some(id) => self.load_project_by_uuid(&mut connection, id).await?,
            None => None,
        };

        let mut task = Task::new(
            create_task_data.title,
            create_task_data.description,
            project,
            create_task_data.due_at_utc,
            create_task_data.deadline_at_utc,
        );
        task.create_record(&mut connection).await?;

        Ok(task)
    }

    pub async fn create_subtask_for_task(
        &self,
        parent_task: Task,
        create_task_data: CreateTaskData,
    ) -> Result<Task, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let project = parent_task.project;

        let mut task = Task::new(
            create_task_data.title,
            create_task_data.description,
            project,
            None,
            None,
        );
        task.parent_task_id = Some(parent_task.id);
        task.create_record(&mut connection).await?;

        Ok(task)
    }

    async fn load_project_by_uuid(
        &self,
        connection: &mut sqlx::pool::PoolConnection<sqlx::Sqlite>,
        id: Uuid,
    ) -> Result<Option<Project>, TaskError> {
        let project = Project::load_by_id(id, connection)
            .await
            .map_err(|_| TaskError::ProjectNotFound)?;
        Ok(project)
    }

    pub async fn load_by_id(&self, task_id: Uuid) -> Result<Option<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        Task::load_by_id(task_id, &mut connection).await
    }

    pub async fn update_task(
        &self,
        task_id: Uuid,
        update_data: UpdatedTaskData,
    ) -> Result<Option<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let task = Task::load_by_id(task_id, &mut connection).await?;

        match task {
            None => Ok(None),
            Some(mut task) => {
                task.update(update_data, &mut connection).await?;
                task.update_record(&mut connection).await?;

                Ok(Some(task))
            }
        }
    }

    pub async fn delete_task(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let task = Task::load_by_id(task_id, &mut connection).await?;

        match task {
            Some(t) => {
                t.delete_record(&mut connection).await?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub async fn load_tasks(&self, include_completed: bool) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let tasks = Task::load_filtered_by_completed(include_completed, &mut connection).await?;

        Ok(tasks)
    }

    pub async fn complete_task(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let task = Task::load_by_id(task_id, &mut connection).await?;

        match task {
            None => Ok(()),
            Some(t) => {
                if let Some(_) = t.completed_at_utc {
                    log::info!("Task was already completed, marking it incomplete");
                    self.unmark_task_completed(t.id).await?;
                    return Ok(());
                }

                let task_subtasks = Task::load_for_parent(t.id, &mut connection).await?;

                for subtask in task_subtasks {
                    self.mark_task_completed(subtask.id).await?;
                }

                self.mark_task_completed(task_id).await
            }
        }
    }

    async fn mark_task_completed(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await.unwrap();
        let task = Task::load_by_id(task_id, &mut connection).await.unwrap();

        match task {
            None => Err(Box::new(TaskError::TaskNotFound)),
            Some(mut t) => {
                t.completed_at_utc = Some(Utc::now());
                t.update_record(&mut connection).await?;

                Ok(())
            }
        }
    }

    async fn unmark_task_completed(&self, task_id: Uuid) -> Result<(), TaskError> {
        let mut connection = self.db_pool.acquire().await.unwrap();
        let task = Task::load_by_id(task_id, &mut connection).await.unwrap();

        match task {
            None => Err(TaskError::TaskNotFound),
            Some(mut t) => {
                t.completed_at_utc = None;
                t.update_record(&mut connection).await.unwrap();

                Ok(())
            }
        }
    }

    pub async fn load_inbox(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let tasks = Task::load_inbox(&mut connection).await?;

        Ok(tasks)
    }

    pub async fn load_due_before(&self, date: DateTime<Utc>) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let tasks = Task::load_due_before(date, &mut connection).await?;

        Ok(tasks)
    }

    pub async fn load_with_deadline(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let tasks = Task::load_with_deadlines(&mut connection).await?;

        Ok(tasks)
    }

    pub async fn load_statistics(
        &self,
    ) -> Result<Vec<super::commands::PeriodTaskStatistic>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let statistics = super::commands::PeriodTaskStatistic::load(&mut connection).await?;

        Ok(statistics)
    }

    pub async fn load_subtasks_for_task(
        &self,
        parent_task_id: Uuid,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let parent_task = Task::load_by_id(parent_task_id, &mut connection)
            .await?
            .unwrap(); // remove this unwrap if parent task doesnt exist anymore (shouldnt happen)

        let subtasks = Task::load_for_parent(parent_task.id, &mut connection).await?;

        Ok(subtasks)
    }

    pub async fn tick(&self, id: Uuid) -> Result<Task, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let task = Task::load_by_id(id, &mut connection).await?;

        match task {
            None => Err(Box::new(TaskError::TaskNotFound)),
            Some(mut t) => {
                t.update_record(&mut connection).await?;

                Ok(t)
            }
        }
    }
}
