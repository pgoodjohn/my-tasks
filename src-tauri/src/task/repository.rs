use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{pool::PoolConnection, Row, Sqlite};
use std::collections::HashMap;
use uuid::Uuid;

use super::Task;
use super::UpdatedTaskData;
use crate::project::Project;

#[async_trait]
pub trait TaskRepository {
    async fn save(&mut self, task: &mut Task) -> Result<(), sqlx::Error>;
    async fn delete(&mut self, task: &Task) -> Result<(), sqlx::Error>;
    async fn find_by_id(&mut self, id: Uuid) -> Result<Option<Task>, sqlx::Error>;
    async fn update_task(
        &mut self,
        task: &mut Task,
        data: UpdatedTaskData,
    ) -> Result<(), sqlx::Error>;
    async fn find_all_filtered_by_completed(
        &mut self,
        include_completed: bool,
    ) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_completed(&mut self) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_by_project(
        &mut self,
        project_id: Uuid,
        include_completed_tasks: bool,
    ) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_by_parent(&mut self, parent_task_id: Uuid) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_completed_by_parent(
        &mut self,
        parent_task_id: Uuid,
    ) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_due_before(&mut self, date: DateTime<Utc>) -> Result<Vec<Task>, sqlx::Error>;
    async fn find_inbox(&mut self) -> Result<Vec<Task>, sqlx::Error>;
    async fn move_subtasks_to_project(
        &mut self,
        parent_task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), sqlx::Error>;
}

pub struct SqliteTaskRepository {
    connection: PoolConnection<Sqlite>,
}

impl SqliteTaskRepository {
    pub fn new(connection: PoolConnection<Sqlite>) -> Self {
        Self { connection }
    }

    async fn row_to_task(&mut self, row: sqlx::sqlite::SqliteRow) -> Result<Task, sqlx::Error> {
        let id = Uuid::parse_str(row.get("id")).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let project_id = match row.get::<Option<String>, _>("project_id") {
            Some(id) => Some(Uuid::parse_str(&id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?),
            None => None,
        };
        let parent_task_id = match row.get::<Option<String>, _>("parent_task_id") {
            Some(id) => Some(Uuid::parse_str(&id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?),
            None => None,
        };
        let due_at_utc = match row.get::<Option<String>, _>("due_at_utc") {
            Some(date) => Some(
                DateTime::parse_from_rfc3339(&date)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                    .with_timezone(&Utc),
            ),
            None => None,
        };
        let created_at_utc = DateTime::parse_from_rfc3339(row.get("created_at_utc"))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc);
        let completed_at_utc = match row.get::<Option<String>, _>("completed_at_utc") {
            Some(date) => Some(
                DateTime::parse_from_rfc3339(&date)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
                    .with_timezone(&Utc),
            ),
            None => None,
        };
        let updated_at_utc = DateTime::parse_from_rfc3339(row.get("updated_at_utc"))
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?
            .with_timezone(&Utc);

        Ok(Task {
            id,
            title: row.get("title"),
            description: row.get("description"),
            project_id,
            project: None,
            parent_task_id,
            due_at_utc,
            created_at_utc,
            completed_at_utc,
            updated_at_utc,
        })
    }

    async fn load_projects_for_tasks(&mut self, tasks: &mut Vec<Task>) -> Result<(), sqlx::Error> {
        let project_ids: Vec<String> = tasks
            .iter()
            .filter_map(|t| t.project_id)
            .map(|id| id.to_string())
            .collect();

        if project_ids.is_empty() {
            return Ok(());
        }

        let placeholders = (0..project_ids.len())
            .map(|i| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(",");

        let query = format!("SELECT * FROM projects WHERE id IN ({})", placeholders);
        let mut query_builder = sqlx::query(&query);

        for id in &project_ids {
            query_builder = query_builder.bind(id);
        }

        let rows = query_builder.fetch_all(&mut *self.connection).await?;

        let mut projects = HashMap::new();
        for row in rows {
            let id: String = row.get("id");
            let project = Project {
                id: Uuid::parse_str(&id).map_err(|e| sqlx::Error::Protocol(e.to_string()))?,
                title: row.get("title"),
                emoji: row.get("emoji"),
                color: row.get("color"),
                description: row.get("description"),
                created_at_utc: DateTime::parse_from_rfc3339(
                    &row.get::<String, _>("created_at_utc"),
                )
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
                .into(),
                updated_at_utc: DateTime::parse_from_rfc3339(
                    &row.get::<String, _>("updated_at_utc"),
                )
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
                .into(),
                archived_at_utc: row
                    .get::<Option<String>, _>("archived_at_utc")
                    .map(|s| DateTime::parse_from_rfc3339(&s))
                    .transpose()
                    .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
                    .map(DateTime::<Utc>::from),
                is_favorite: row.get("is_favorite"),
            };
            projects.insert(project.id, project);
        }

        for task in tasks {
            if let Some(project_id) = task.project_id {
                task.project = projects.get(&project_id).cloned();
            }
        }

        Ok(())
    }
}

#[async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn save(&mut self, task: &mut Task) -> Result<(), sqlx::Error> {
        let exists = sqlx::query("SELECT 1 FROM tasks WHERE id = ?1 LIMIT 1")
            .bind(task.id.to_string())
            .fetch_optional(&mut *self.connection)
            .await?
            .is_some();

        task.updated_at_utc = Utc::now();

        if exists {
            sqlx::query(
                "UPDATE tasks SET title = ?1, description = ?2, due_at_utc = ?3, parent_task_id = ?4, updated_at_utc = ?5, project_id = ?6, completed_at_utc = ?7 WHERE id = ?8"
            )
            .bind(&task.title)
            .bind(&task.description)
            .bind(task.due_at_utc.map(|date| date.to_rfc3339()))
            .bind(task.parent_task_id.map(|task_uuid| task_uuid.to_string()))
            .bind(task.updated_at_utc.to_rfc3339())
            .bind(task.project_id.map(|project_id| project_id.to_string()))
            .bind(task.completed_at_utc.map(|date| date.to_rfc3339()))
            .bind(task.id.to_string())
            .execute(&mut *self.connection)
            .await?;
        } else {
            sqlx::query(
                "INSERT INTO tasks (id, title, description, project_id, parent_task_id, due_at_utc, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
            )
            .bind(task.id.to_string())
            .bind(&task.title)
            .bind(&task.description)
            .bind(task.project_id.map(|project_id| project_id.to_string()))
            .bind(task.parent_task_id.map(|id| id.to_string()))
            .bind(task.due_at_utc.map(|date| date.to_rfc3339()))
            .bind(task.created_at_utc.to_rfc3339())
            .bind(task.updated_at_utc.to_rfc3339())
            .execute(&mut *self.connection)
            .await?;
        }

        Ok(())
    }

    async fn delete(&mut self, task: &Task) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tasks WHERE id = ?1")
            .bind(task.id.to_string())
            .execute(&mut *self.connection)
            .await?;

        Ok(())
    }

    async fn find_by_id(&mut self, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM tasks WHERE id = ?1 LIMIT 1")
            .bind(id.to_string())
            .fetch_optional(&mut *self.connection)
            .await?;

        match row {
            Some(row) => Ok(Some(self.row_to_task(row).await?)),
            None => Ok(None),
        }
    }

    async fn find_all_filtered_by_completed(
        &mut self,
        include_completed: bool,
    ) -> Result<Vec<Task>, sqlx::Error> {
        let query = match include_completed {
            true => "SELECT * FROM tasks ORDER BY updated_at_utc DESC",
            false => {
                "SELECT * FROM tasks WHERE completed_at_utc IS NULL ORDER BY updated_at_utc DESC"
            }
        };

        let rows = sqlx::query(query).fetch_all(&mut *self.connection).await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_completed(&mut self) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE completed_at_utc IS NOT NULL ORDER BY completed_at_utc DESC",
        )
        .fetch_all(&mut *self.connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_by_project(
        &mut self,
        project_id: Uuid,
        include_completed_tasks: bool,
    ) -> Result<Vec<Task>, sqlx::Error> {
        let mut query = "SELECT * FROM tasks WHERE project_id = ?1".to_string();
        if !include_completed_tasks {
            query += " AND completed_at_utc IS NULL";
        }
        query += " ORDER BY updated_at_utc DESC";

        let rows = sqlx::query(&query)
            .bind(project_id.to_string())
            .fetch_all(&mut *self.connection)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_by_parent(&mut self, parent_task_id: Uuid) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE parent_task_id = ?1 AND completed_at_utc IS NULL ORDER BY updated_at_utc DESC",
        )
        .bind(parent_task_id.to_string())
        .fetch_all(&mut *self.connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_completed_by_parent(
        &mut self,
        parent_task_id: Uuid,
    ) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE parent_task_id = ?1 AND completed_at_utc IS NOT NULL ORDER BY completed_at_utc DESC",
        )
        .bind(parent_task_id.to_string())
        .fetch_all(&mut *self.connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_due_before(&mut self, date: DateTime<Utc>) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE due_at_utc < ?1 AND completed_at_utc IS NULL ORDER BY due_at_utc ASC"
        )
        .bind(date.to_rfc3339())
        .fetch_all(&mut *self.connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn find_inbox(&mut self) -> Result<Vec<Task>, sqlx::Error> {
        let rows = sqlx::query(
            "SELECT * FROM tasks WHERE project_id IS NULL AND completed_at_utc IS NULL ORDER BY created_at_utc DESC"
        )
        .fetch_all(&mut *self.connection)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(self.row_to_task(row).await?);
        }

        self.load_projects_for_tasks(&mut tasks).await?;

        Ok(tasks)
    }

    async fn update_task(
        &mut self,
        task: &mut Task,
        data: UpdatedTaskData,
    ) -> Result<(), sqlx::Error> {
        task.title = data.title;
        task.description = data.description;
        task.due_at_utc = data
            .due_date
            .map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
        task.updated_at_utc = Utc::now();

        if let Some(project_id) = data.project_id {
            let project_uuid =
                Uuid::parse_str(&project_id).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
            task.project_id = Some(project_uuid);
        } else {
            task.project_id = None;
        }

        self.save(task).await
    }

    async fn move_subtasks_to_project(
        &mut self,
        parent_task_id: Uuid,
        project_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE tasks SET project_id = ?1, parent_task_id = NULL WHERE parent_task_id = ?2",
        )
        .bind(project_id.to_string())
        .bind(parent_task_id.to_string())
        .execute(&mut *self.connection)
        .await?;

        Ok(())
    }
}
