use super::repository::TaskRepository;
use super::{CreateTaskData, PeriodTaskStatistic, Task, UpdatedTaskData};
use crate::recurring_task::manager::RecurringTaskManager;
use crate::repository::RepositoryProvider;
use chrono::{DateTime, Utc};
use std::error::Error;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task not found")]
    TaskNotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct TaskManager<'a> {
    repository_provider: &'a RepositoryProvider,
}

impl<'a> TaskManager<'a> {
    pub fn new(repository_provider: &'a RepositoryProvider) -> Self {
        Self {
            repository_provider,
        }
    }

    pub async fn create_task(
        &self,
        create_task_data: CreateTaskData,
    ) -> Result<Task, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        let mut task = Task::new(
            create_task_data.title,
            create_task_data.description,
            create_task_data
                .project_id
                .as_ref()
                .map(|id| Uuid::parse_str(id))
                .transpose()?,
            None, // parent_task_id
            create_task_data
                .due_at_utc
                .as_ref()
                .map(|date| DateTime::parse_from_rfc3339(date))
                .transpose()?
                .map(DateTime::<Utc>::from),
        );

        repository.save(&mut task).await?;
        Ok(task)
    }

    pub async fn create_subtask_for_task(
        &self,
        parent_task: Task,
        create_task_data: CreateTaskData,
    ) -> Result<Task, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;

        let mut task = Task::new(
            create_task_data.title,
            create_task_data.description,
            parent_task.project_id,
            Some(parent_task.id),
            create_task_data
                .due_at_utc
                .as_ref()
                .map(|date| DateTime::parse_from_rfc3339(date))
                .transpose()?
                .map(DateTime::<Utc>::from),
        );

        repository.save(&mut task).await?;
        Ok(task)
    }

    pub async fn load_by_id(&self, task_id: Uuid) -> Result<Option<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository.find_by_id(task_id).await.map_err(Into::into)
    }

    pub async fn update_task(
        &self,
        task_id: Uuid,
        update_data: UpdatedTaskData,
    ) -> Result<Option<Task>, Box<dyn Error>> {
        let mut task_repository = self.repository_provider.task_repository().await?;
        let mut recurring_task_repository =
            self.repository_provider.recurring_task_repository().await?;

        let mut task = match task_repository.find_by_id(task_id).await? {
            None => return Ok(None),
            Some(task) => task,
        };

        // If the due date is being updated, handle recurring task update
        if let Some(due_date) = &update_data.due_date {
            let new_due_date = DateTime::parse_from_rfc3339(due_date)?.with_timezone(&Utc);
            let mut recurring_task_manager =
                RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);
            recurring_task_manager
                .handle_task_update(task_id, new_due_date)
                .await?;
        }

        task_repository.update_task(&mut task, update_data).await?;
        Ok(Some(task))
    }

    pub async fn delete_task(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;

        let task = match repository.find_by_id(task_id).await? {
            Some(task) => task,
            None => return Ok(()),
        };

        repository.delete(&task).await?;
        Ok(())
    }

    pub async fn load_tasks(&self, include_completed: bool) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository
            .find_all_filtered_by_completed(include_completed)
            .await
            .map_err(Into::into)
    }

    pub async fn load_tasks_by_project(
        &self,
        project_id: Uuid,
        include_completed: bool,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository
            .find_by_project(project_id, include_completed)
            .await
            .map_err(Into::into)
    }

    pub async fn complete_task(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut task_repository = self.repository_provider.task_repository().await?;
        let mut recurring_task_repository =
            self.repository_provider.recurring_task_repository().await?;

        let mut task = match task_repository.find_by_id(task_id).await? {
            None => return Ok(()),
            Some(task) => task,
        };

        if task.completed_at_utc.is_some() {
            log::info!("Task was already completed, marking it incomplete");
            self.unmark_task_completed(task.id).await?;
            return Ok(());
        }

        let task_subtasks = task_repository.find_by_parent(task.id).await?;

        for mut subtask in task_subtasks {
            subtask.completed_at_utc = Some(Utc::now());
            task_repository.save(&mut subtask).await?;
        }

        task.completed_at_utc = Some(Utc::now());
        task_repository.save(&mut task).await?;

        // Handle recurring task if it exists
        let mut recurring_task_manager =
            RecurringTaskManager::new(&mut recurring_task_repository, &mut task_repository);
        recurring_task_manager.handle_task_completion(&task).await?;

        Ok(())
    }

    async fn unmark_task_completed(&self, task_id: Uuid) -> Result<(), TaskError> {
        let mut repository = self.repository_provider.task_repository().await?;

        let mut task = match repository.find_by_id(task_id).await? {
            None => return Err(TaskError::TaskNotFound),
            Some(task) => task,
        };

        task.completed_at_utc = None;
        repository.save(&mut task).await?;

        Ok(())
    }

    pub async fn load_inbox(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository.find_inbox().await.map_err(Into::into)
    }

    pub async fn load_due_before(&self, date: DateTime<Utc>) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository.find_due_before(date).await.map_err(Into::into)
    }

    pub async fn load_completed_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository.find_completed().await.map_err(Into::into)
    }

    pub async fn load_statistics(&self) -> Result<Vec<PeriodTaskStatistic>, Box<dyn Error>> {
        let db_pool = &self.repository_provider.pool;
        PeriodTaskStatistic::load(db_pool).await
    }

    pub async fn load_subtasks_for_task(
        &self,
        parent_task_id: Uuid,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository
            .find_by_parent(parent_task_id)
            .await
            .map_err(Into::into)
    }

    pub async fn load_completed_subtasks_for_task(
        &self,
        parent_task_id: Uuid,
    ) -> Result<Vec<Task>, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository
            .find_completed_by_parent(parent_task_id)
            .await
            .map_err(Into::into)
    }

    pub async fn move_subtasks_to_project(
        &self,
        parent_task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut repository = self.repository_provider.task_repository().await?;
        repository
            .move_subtasks_to_project(parent_task_id, project_id)
            .await
    }

    pub async fn load_task(&self, task_id: Uuid) -> Result<Task, Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        Ok(repository
            .find_by_id(task_id)
            .await?
            .ok_or_else(|| Box::new(TaskError::TaskNotFound))?)
    }

    pub async fn archive_task(&self, task_id: Uuid) -> Result<(), Box<dyn Error>> {
        let mut repository = self.repository_provider.task_repository().await?;
        let mut task = repository
            .find_by_id(task_id)
            .await?
            .ok_or_else(|| Box::new(TaskError::TaskNotFound))?;

        task.completed_at_utc = Some(Utc::now());
        task.updated_at_utc = Utc::now();
        repository.save(&mut task).await?;
        Ok(())
    }
}
