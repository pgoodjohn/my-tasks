use chrono::{DateTime, Datelike, Duration, Utc, Weekday};
use uuid::Uuid;

use crate::task::repository::TaskRepository;
use crate::task::Task;

use super::repository::RecurringTaskRepository;
use super::{Frequency, RecurringTask, WeekDay};

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
        weekdays: Option<Vec<WeekDay>>,
    ) -> Result<RecurringTask, Box<dyn std::error::Error>> {
        let task = self.task_repository.find_by_id(task_id).await?;
        let task = task.ok_or("Task not found")?;

        // Check if a recurring task already exists
        if let Some(_) = self
            .recurring_task_repository
            .find_by_task_id(task_id)
            .await?
        {
            return Err("Task already has recurring settings".into());
        }

        // Validate weekdays are only provided for weekly frequency
        if frequency != Frequency::Weekly && weekdays.is_some() {
            return Err("Weekdays can only be specified for weekly frequency".into());
        }

        let base_date = task.due_at_utc.unwrap_or_else(Utc::now);
        let next_due_at_utc = self.calculate_due_date_for_base_date_and_frequency(
            base_date,
            &frequency,
            interval,
            weekdays.as_ref(),
        )?;

        let mut recurring_task =
            RecurringTask::new(task_id, frequency, interval, weekdays, next_due_at_utc);
        self.recurring_task_repository
            .save(&mut recurring_task)
            .await?;
        Ok(recurring_task)
    }

    pub async fn delete_recurring_task(
        &mut self,
        task_id: Uuid,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.recurring_task_repository
            .delete_by_task_id(task_id)
            .await?;
        Ok(())
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
            // Create a new task for the next occurrence
            let mut new_task = Task::new(
                task.title.clone(),
                task.description.clone(),
                task.project_id,
                task.parent_task_id,
                Some(recurring_task.next_due_at_utc),
            );

            // Calculate the next due date based on frequency and interval
            let frequency = recurring_task.frequency()?;
            let next_due_date = self.calculate_due_date_for_base_date_and_frequency(
                recurring_task.next_due_at_utc,
                &frequency,
                recurring_task.interval,
                recurring_task.weekdays().as_ref(),
            )?;

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

    pub async fn handle_task_update(
        &mut self,
        task_id: Uuid,
        new_due_date: DateTime<Utc>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if this task has a recurring configuration
        if let Some(mut recurring_task) = self
            .recurring_task_repository
            .find_by_task_id(task_id)
            .await?
        {
            recurring_task.next_due_at_utc = self.calculate_due_date_for_base_date_and_frequency(
                new_due_date,
                &recurring_task.frequency()?,
                recurring_task.interval,
                recurring_task.weekdays().as_ref(),
            )?;
            self.recurring_task_repository
                .save(&mut recurring_task)
                .await?;
        }
        Ok(())
    }

    pub async fn update_recurring_task(
        &mut self,
        task_id: Uuid,
        frequency: Frequency,
        interval: i32,
        weekdays: Option<Vec<WeekDay>>,
    ) -> Result<RecurringTask, Box<dyn std::error::Error>> {
        let mut recurring_task = self
            .recurring_task_repository
            .find_by_task_id(task_id)
            .await?
            .ok_or("No recurring task found for this task")?;

        // Validate weekdays are only provided for weekly frequency
        if frequency != Frequency::Weekly && weekdays.is_some() {
            return Err("Weekdays can only be specified for weekly frequency".into());
        }

        let task = self.task_repository.find_by_id(task_id).await?;
        let task = task.ok_or("Task not found")?;

        let base_date = task.due_at_utc.unwrap_or_else(Utc::now);
        let next_due_at_utc = self.calculate_due_date_for_base_date_and_frequency(
            base_date,
            &frequency,
            interval,
            weekdays.as_ref(),
        )?;

        recurring_task.frequency = frequency.to_string();
        recurring_task.interval = interval;
        recurring_task.weekdays = weekdays.map(|days| {
            days.iter()
                .map(|day| day.to_string())
                .collect::<Vec<_>>()
                .join(",")
        });
        recurring_task.next_due_at_utc = next_due_at_utc;

        self.recurring_task_repository
            .save(&mut recurring_task)
            .await?;
        Ok(recurring_task)
    }

    fn calculate_due_date_for_base_date_and_frequency(
        &self,
        base_date: DateTime<Utc>,
        frequency: &Frequency,
        interval: i32,
        weekdays: Option<&Vec<WeekDay>>,
    ) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
        let next_due = match frequency {
            Frequency::Daily => base_date + Duration::days(interval as i64),
            Frequency::Weekly => {
                if let Some(weekdays) = weekdays {
                    if weekdays.is_empty() {
                        return Err(
                            "At least one weekday must be specified for weekly recurring tasks"
                                .into(),
                        );
                    }

                    // Convert weekdays to chrono::Weekday
                    let weekday_numbers: Vec<u32> = weekdays
                        .iter()
                        .map(|day| match day {
                            WeekDay::Monday => 1,
                            WeekDay::Tuesday => 2,
                            WeekDay::Wednesday => 3,
                            WeekDay::Thursday => 4,
                            WeekDay::Friday => 5,
                            WeekDay::Saturday => 6,
                            WeekDay::Sunday => 7,
                        })
                        .collect();

                    // Find the next occurrence after base_date
                    let mut current = base_date + Duration::days(1); // Start from tomorrow
                    while !weekday_numbers.contains(&current.weekday().number_from_monday()) {
                        current = current + Duration::days(1);
                    }

                    // Add weeks based on interval (subtract 1 since we already moved to next occurrence)
                    current + Duration::weeks((interval - 1) as i64)
                } else {
                    base_date + Duration::weeks(interval as i64)
                }
            }
            Frequency::Monthly => {
                // Add months by adjusting the month number
                let total_months = base_date.month0() as i32 + interval;
                let years_to_add = total_months / 12;
                let final_month = (total_months % 12) as u32;

                base_date
                    .date_naive()
                    .with_month0(final_month)
                    .and_then(|d| d.with_year(base_date.year() + years_to_add))
                    .map(|d| DateTime::from_naive_utc_and_offset(d.and_time(base_date.time()), Utc))
                    .unwrap_or(base_date)
            }
            Frequency::Yearly => base_date
                .date_naive()
                .with_year(base_date.year() + interval)
                .map(|d| DateTime::from_naive_utc_and_offset(d.and_time(base_date.time()), Utc))
                .unwrap_or(base_date),
        };

        Ok(next_due)
    }
}
