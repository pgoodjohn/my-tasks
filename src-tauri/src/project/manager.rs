use super::repository::ProjectRepository;
use super::Project;
use super::ProjectDetail;
use crate::task::repository::TaskRepository;
use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

pub struct ProjectsManager<'a> {
    project_repository: &'a mut dyn ProjectRepository,
    task_repository: &'a mut dyn TaskRepository,
}

impl<'a> ProjectsManager<'a> {
    pub fn new(
        project_repository: &'a mut dyn ProjectRepository,
        task_repository: &'a mut dyn TaskRepository,
    ) -> Self {
        ProjectsManager {
            project_repository,
            task_repository,
        }
    }

    pub async fn load_all(
        &mut self,
        show_archived_projects: bool,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let projects = if show_archived_projects {
            self.project_repository.find_all().await?
        } else {
            self.project_repository.find_not_archived().await?
        };
        Ok(projects)
    }

    pub async fn load_project_detail(
        &mut self,
        project_id: Uuid,
        include_completed_tasks: bool,
    ) -> Result<ProjectDetail, Box<dyn Error>> {
        let project = self
            .project_repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        let tasks = self
            .task_repository
            .find_by_project(project_id, include_completed_tasks)
            .await?;

        let project_detail = ProjectDetail { project, tasks };
        Ok(project_detail)
    }

    pub async fn create_project(
        &mut self,
        title: String,
        description: Option<String>,
    ) -> Result<Project, Box<dyn Error>> {
        let mut project = Project {
            id: Uuid::now_v7(),
            title,
            emoji: None,
            color: None,
            description,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            archived_at_utc: None,
            is_favorite: false,
        };

        self.project_repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn update_project(
        &mut self,
        project_id: Uuid,
        new_title: String,
        new_emoji: Option<String>,
        new_color: Option<String>,
        new_description: Option<String>,
    ) -> Result<Project, Box<dyn Error>> {
        let mut project = self
            .project_repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.title = new_title;
        project.emoji = new_emoji;
        project.color = new_color;
        project.description = new_description;
        project.updated_at_utc = Utc::now();

        self.project_repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn archive_project(&mut self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut project = self
            .project_repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.archived_at_utc = Some(Utc::now());
        project.updated_at_utc = Utc::now();

        self.project_repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn count_open_tasks(&mut self, project_id: Uuid) -> Result<i64, Box<dyn Error>> {
        self.project_repository
            .count_open_tasks(project_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_favorite(&mut self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut project = self
            .project_repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.is_favorite = true;
        project.updated_at_utc = Utc::now();

        self.project_repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn remove_favorite(&mut self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut project = self
            .project_repository
            .find_by_id(project_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Project not found")?;

        project.is_favorite = false;
        project.updated_at_utc = Utc::now();

        self.project_repository
            .save(&mut project)
            .await
            .map_err(|e| e.to_string())?;

        Ok(project)
    }

    pub async fn load_favorites(&mut self) -> Result<Vec<Project>, Box<dyn Error>> {
        self.project_repository
            .find_favorites()
            .await
            .map_err(|e| e.into())
    }
}
