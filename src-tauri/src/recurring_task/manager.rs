use chrono::{DateTime, Days, Months, Utc};
use uuid::Uuid;

use crate::task::repository::TaskRepository;
use crate::task::{CreateTaskData, Task};

use super::repository::RecurringTaskRepository;
use super::{Frequency, RecurringTask};

pub struct RecurringTaskManager<'a> {
    recurring_task_repository: &'a mut dyn RecurringTaskRepository,
    task_repository: &'a mut dyn TaskRepository,
}

impl<'a> RecurringTaskManager<'a> {
    pub fn new(
        recurring_task_repository: &'a mut dyn RecurringTaskRepository,
        task_repository: &'a mut dyn TaskRepository,
    ) -> Self {
        Self {
            recurring_task_repository,
            task_repository,
        }
    }

    pub async fn setup_recurring_task(
        &mut self,
        task_id: Uuid,
        frequency: Frequency,
        interval: i32,
        first_due_date: DateTime<Utc>,
    ) -> Result<RecurringTask, Box<dyn std::error::Error>> {
        let mut recurring_task = RecurringTask::new(task_id, frequency, interval, first_due_date);
        self.recurring_task_repository
            .save(&mut recurring_task)
            .await?;
        Ok(recurring_task)
    }

    pub async fn handle_task_completion(
        &mut self,
        task: &Task,
    ) -> Result<Option<Task>, Box<dyn std::error::Error>> {
        // Check if this task has a recurring configuration
        if let Some(mut recurring_task) = self
            .recurring_task_repository
            .find_by_task_id(task.id)
            .await?
        {
            // Calculate the next due date based on frequency and interval
            let next_due_date = self.calculate_next_due_date(&recurring_task)?;

            // Create a new task for the next occurrence
            let mut new_task = Task::new(
                task.title.clone(),
                task.description.clone(),
                task.project_id,
                task.parent_task_id,
                Some(next_due_date),
            );

            // Save the new task
            self.task_repository.save(&mut new_task).await?;

            // Update the recurring task with the new task_id and next_due_date
            recurring_task.task_id = new_task.id;
            recurring_task.next_due_at_utc = next_due_date;
            self.recurring_task_repository
                .save(&mut recurring_task)
                .await?;

            Ok(Some(new_task))
        } else {
            Ok(None)
        }
    }

    fn calculate_next_due_date(
        &self,
        recurring_task: &RecurringTask,
    ) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        let frequency = recurring_task.frequency()?;
        let interval = recurring_task.interval;

        let next_due = match frequency {
            Frequency::Daily => recurring_task.next_due_at_utc + Days::new(interval as u64),
            Frequency::Weekly => recurring_task.next_due_at_utc + Days::new((7 * interval) as u64),
            Frequency::Monthly => recurring_task.next_due_at_utc + Months::new(interval as u32),
            Frequency::Yearly => {
                recurring_task.next_due_at_utc + Months::new((12 * interval) as u32)
            }
        };

        Ok(next_due)
    }
}
