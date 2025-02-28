use std::error::Error;

use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use super::Project;
use super::ProjectDetail;
use crate::task::Task;

pub struct ProjectsManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> ProjectsManager<'a> {
    pub fn new(db_pool: &'a SqlitePool) -> Self {
        ProjectsManager { db_pool }
    }

    pub async fn load_projects(
        &self,
        show_archived_projects: bool,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        match show_archived_projects {
            true => {
                let projects = Project::list_all_projects(&mut connection).await?;
                Ok(projects)
            }
            false => {
                let projects = Project::list_not_archived_projects(&mut connection).await?;
                Ok(projects)
            }
        }
    }

    pub async fn load_project_details(
        &self,
        project_id: Uuid,
        include_completed_tasks: bool,
    ) -> Result<ProjectDetail, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let project = Project::load_by_id(project_id, &mut connection)
            .await?
            .unwrap();

        let tasks = Task::load_for_project(project.id, &mut connection).await?;

        let project_detail = ProjectDetail { project, tasks };

        if include_completed_tasks {
            return Ok(project_detail);
        }

        let open_tasks: Vec<Task> = project_detail
            .tasks
            .into_iter()
            .filter(|task| task.completed_at_utc.is_none())
            .collect();

        let open_project_detail = ProjectDetail {
            project: project_detail.project,
            tasks: open_tasks,
        };

        Ok(open_project_detail)
    }

    pub async fn create_project(
        &self,
        title: String,
        emoji: Option<String>,
        color: Option<String>,
        description: Option<String>,
    ) -> Result<Project, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let mut project = Project::new(title, emoji, color, description);

        project.save(&mut connection).await?;

        Ok(project)
    }

    pub async fn update_project(
        &self,
        project_id: Uuid,
        new_title: Option<String>,
        new_emoji: Option<String>,
        new_color: Option<String>,
        new_description: Option<String>,
    ) -> Result<Project, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let mut project = Project::load_by_id(project_id, &mut connection)
            .await?
            .unwrap(); // TODO: remove unwrap for option

        project.title = new_title.unwrap_or(project.title);
        project.emoji = new_emoji;
        project.description = new_description;
        project.color = new_color;
        project.updated_at_utc = Utc::now();

        project.save(&mut connection).await?;

        Ok(project)
    }

    pub async fn archive_project(&self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let project = Project::load_by_id(project_id, &mut connection).await?;

        match project {
            None => Err(Box::<dyn Error>::from("Project not found")),
            Some(mut project) => {
                project.archived_at_utc = Some(Utc::now());
                project.save(&mut connection).await?;

                Ok(project)
            }
        }
    }

    pub async fn count_open_tasks(&self, project_id: Uuid) -> Result<i64, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let project = Project::load_by_id(project_id, &mut connection).await?;

        match project {
            Some(project) => {
                let count = project
                    .count_open_tasks_for_project(&mut connection)
                    .await?;
                Ok(count)
            }
            None => Err(Box::<dyn Error>::from("Could not find project")),
        }
    }

    pub async fn add_favorite(&self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let project = Project::load_by_id(project_id, &mut connection).await?;

        match project {
            Some(mut project) => {
                project.is_favorite = true;
                project.update_record(&mut connection).await?;

                Ok(project)
            }
            None => Err(Box::<dyn Error>::from("Could not find project")),
        }
    }

    pub async fn remove_favorite(&self, project_id: Uuid) -> Result<Project, String> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap();

        match project {
            None => Err("Project not found".to_string()),
            Some(mut project) => {
                project.is_favorite = false;
                project.update_record(&mut connection).await.unwrap();

                Ok(project)
            }
        }
    }

    pub async fn load_favorites(&self) -> Result<Vec<Project>, Box<dyn Error>> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await?;

        let projects = Project::list_favorite_projects(&mut connection).await?;

        Ok(projects)
    }
}
