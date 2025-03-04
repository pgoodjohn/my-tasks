use super::repository::ProjectRepository;
use super::Project;
use super::ProjectDetail;
use crate::repository::RepositoryProvider;
use crate::task::Task;
use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

pub struct ProjectsManager<'a> {
    repository_provider: &'a RepositoryProvider,
}

impl<'a> ProjectsManager<'a> {
    pub fn new(repository_provider: &'a RepositoryProvider) -> Self {
        ProjectsManager {
            repository_provider,
        }
    }

    pub async fn load_all(
        &self,
        show_archived_projects: bool,
    ) -> Result<Vec<Project>, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;
        let projects = if show_archived_projects {
            repository.find_all().await?
        } else {
            repository.find_not_archived().await?
        };
        Ok(projects)
    }

    pub async fn load_project_detail(
        &self,
        project_id: Uuid,
        _include_completed_tasks: bool,
    ) -> Result<ProjectDetail, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;

        let project = repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        let tasks = Task::load_for_project(project.id, &self.repository_provider.pool).await?;

        let project_detail = ProjectDetail { project, tasks };
        Ok(project_detail)
    }

    pub async fn create_project(
        &self,
        title: String,
        description: Option<String>,
    ) -> Result<Project, sqlx::Error> {
        let mut repository = self.repository_provider.project_repository().await?;

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

        repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn update_project(
        &self,
        project_id: Uuid,
        new_title: String,
        new_emoji: Option<String>,
        new_color: Option<String>,
        new_description: Option<String>,
    ) -> Result<Project, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;

        let mut project = repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.title = new_title;
        project.emoji = new_emoji;
        project.color = new_color;
        project.description = new_description;
        project.updated_at_utc = Utc::now();

        repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn archive_project(&self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;

        let mut project = repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.archived_at_utc = Some(Utc::now());
        project.updated_at_utc = Utc::now();

        repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn count_open_tasks(&self, project_id: Uuid) -> Result<i64, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;
        repository
            .count_open_tasks(project_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_favorite(&self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;

        let mut project = repository
            .find_by_id(project_id)
            .await?
            .ok_or("Project not found")?;

        project.is_favorite = true;
        project.updated_at_utc = Utc::now();

        repository.save(&mut project).await?;

        Ok(project)
    }

    pub async fn remove_favorite(&self, project_id: Uuid) -> Result<Project, Box<dyn Error>> {
        let mut repository = self
            .repository_provider
            .project_repository()
            .await
            .map_err(|e| e.to_string())?;

        let mut project = repository
            .find_by_id(project_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Project not found")?;

        project.is_favorite = false;
        project.updated_at_utc = Utc::now();

        repository
            .save(&mut project)
            .await
            .map_err(|e| e.to_string())?;

        Ok(project)
    }

    pub async fn load_favorites(&self) -> Result<Vec<Project>, Box<dyn Error>> {
        let mut repository = self.repository_provider.project_repository().await?;
        repository.find_favorites().await.map_err(|e| e.into())
    }
}
