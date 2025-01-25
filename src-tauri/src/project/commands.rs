use chrono::Utc;
use sqlx::SqlitePool;
use tauri::async_runtime::Mutex;
use tauri::State;
use uuid::Uuid;

use super::Project;
use super::ProjectDetail;
use crate::commands::ErrorResponse;
use crate::configuration::Configuration;
use crate::task::Task;

pub struct ProjectsManager<'a> {
    db_pool: &'a SqlitePool,
    configuration: &'a Mutex<Configuration>,
}

impl<'a> ProjectsManager<'a> {
    pub fn new(
        db_pool: &'a SqlitePool,
        configuration: &'a Mutex<Configuration>,
    ) -> Result<Self, ()> {
        Ok(ProjectsManager {
            db_pool,
            configuration,
        })
    }

    pub async fn load_projects(&self, show_archived_projects: bool) -> Result<Vec<Project>, ()> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        match show_archived_projects {
            true => {
                let projects = Project::list_all_projects(&mut connection).await?;
                return Ok(projects);
            }
            false => {
                let projects = Project::list_not_archived_projects(&mut connection).await?;
                return Ok(projects);
            }
        }
    }

    pub async fn load_project_details(
        &self,
        project_id: Uuid,
        include_completed_tasks: bool,
    ) -> Result<ProjectDetail, ()> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap()
            .unwrap();

        let tasks = Task::load_for_project(project.id, &mut connection)
            .await
            .unwrap();

        let project_detail = ProjectDetail {
            project: project,
            tasks: tasks,
        };

        if include_completed_tasks {
            return Ok(project_detail);
        }

        let open_tasks: Vec<Task> = project_detail
            .tasks
            .into_iter()
            .filter(|task| !task.completed_at_utc.is_some())
            .collect();

        let open_project_detail = ProjectDetail {
            project: project_detail.project,
            tasks: open_tasks,
        };

        return Ok(open_project_detail);
    }

    pub async fn create_project(
        &self,
        title: String,
        emoji: Option<String>,
        color: Option<String>,
        description: Option<String>,
    ) -> Result<Project, ()> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let mut project = Project::new(title, emoji, color, description);

        project.save(&mut connection).await.unwrap();

        return Ok(project);
    }

    pub async fn update_project(
        &self,
        project_id: Uuid,
        new_title: Option<String>,
        new_emoji: Option<String>,
        new_color: Option<String>,
        new_description: Option<String>,
    ) -> Result<Project, ()> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let mut project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap()
            .unwrap();

        project.title = new_title.unwrap_or(project.title);
        project.emoji = new_emoji;
        project.description = new_description;
        project.color = new_color;
        project.updated_at_utc = Utc::now();

        project.save(&mut connection).await.unwrap();

        return Ok(project);
    }

    pub async fn archive_project(&self, project_id: Uuid) -> Result<Project, String> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap();

        match project {
            None => {
                return Err("Project not found".to_string());
            }
            Some(mut project) => {
                project.archived_at_utc = Some(Utc::now());
                project.save(&mut connection).await.unwrap();

                let mut locked_configuration = self.configuration.try_lock().unwrap();

                locked_configuration.remove_favorite_project(&project.id.to_string());
                locked_configuration.save().unwrap();

                return Ok(project);
            }
        }
    }

    pub async fn count_open_tasks(&self, project_id: Uuid) -> Result<i64, String> {
        let mut connection: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.db_pool.acquire().await.unwrap();

        let project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap();

        match project {
            Some(project) => {
                let count = project
                    .count_open_tasks_for_project(&mut connection)
                    .await
                    .unwrap();
                return Ok(count);
            }
            None => {
                return Err("Could not find project".to_string());
            }
        }
    }

    pub async fn favorite_project(&self, project_id: Uuid) -> Result<Project, String> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let project = Project::load_by_id(project_id, &mut connection)
            .await
            .unwrap();

        match project {
            Some(project) => {
                let mut locked_configuration = self.configuration.try_lock().unwrap();
                locked_configuration
                    .favorite_projects_uuids
                    .push(project.id.to_string());

                locked_configuration.save();

                return Ok(project);
            }
            None => {
                return Err("Could not find project".to_string());
            }
        }
    }
}

#[tauri::command]
pub async fn load_projects_command(
    show_archived_projects: bool,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!("Running list projects command");
    let projects_manager = ProjectsManager::new(&db, &configuration).unwrap();

    match projects_manager.load_projects(show_archived_projects).await {
        Ok(projects) => Ok(serde_json::to_string(&projects).unwrap()),
        Err(_) => {
            let error = ErrorResponse::new(
                "load_projects_command".to_string(),
                "Failed to load projects".to_string(),
                "Failed to load projects".to_string(),
            );
            Err(serde_json::to_string(&error).unwrap())
        }
    }
}

#[tauri::command]
pub async fn create_project_command(
    title: String,
    emoji: Option<String>,
    color: Option<String>,
    description: Option<String>,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!(
        "Running create project command for: {:?} | {:?}",
        title,
        description
    );

    let projects_manager = ProjectsManager::new(&db, &configuration).unwrap();
    let project = projects_manager
        .create_project(title, emoji, color, description)
        .await
        .unwrap();

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn update_project_command(
    project_id: String,
    new_title: Option<String>,
    new_emoji: Option<String>,
    new_color: Option<String>,
    new_description: Option<String>,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!(
        "Running update project command for: {:?} | {:?}",
        new_title,
        new_description
    );

    let projects_manager = ProjectsManager::new(&db, &configuration).unwrap();

    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    let project = projects_manager
        .update_project(
            project_uuid,
            new_title,
            new_emoji,
            new_color,
            new_description,
        )
        .await
        .unwrap();

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn archive_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<crate::configuration::Configuration>>,
) -> Result<String, String> {
    log::debug!(
        "Running archive project command for project ID: {}",
        project_id
    );

    let projects_manager = ProjectsManager::new(&db, &configuration).unwrap();

    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    // let mut locked_configuration = configuration.lock().await;

    let project = projects_manager
        .archive_project(project_uuid)
        .await
        .unwrap();

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub async fn load_project_details_command(
    project_id: String,
    include_completed_tasks: bool,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!(
        "Running load project details command for project ID: {}, include_completed_tasks: {:?}",
        project_id,
        include_completed_tasks
    );

    let projects_manager = ProjectsManager::new(&db, &configuration).unwrap();

    let project_uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    match projects_manager
        .load_project_details(project_uuid, include_completed_tasks)
        .await
    {
        Ok(project_detail) => Ok(serde_json::to_string(&project_detail).unwrap()),
        Err(_) => {
            let error = ErrorResponse::new(
                "load_project_details_command".to_string(),
                "Failed to load project details".to_string(),
                "Failed to load project details".to_string(),
            );
            Err(serde_json::to_string(&error).unwrap())
        }
    }
}

#[tauri::command]
pub async fn count_open_tasks_for_project_command(
    project_id: String,
    db: State<'_, SqlitePool>,
    configuration: State<'_, Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!(
        "Running count open tasks for project command for project ID: {}",
        project_id
    );

    let manager = ProjectsManager::new(&db, &configuration).unwrap();

    let uuid = Uuid::parse_str(&project_id)
        .map_err(|e| e.to_string())
        .unwrap();

    Ok(manager.count_open_tasks(uuid).await.unwrap().to_string())
}
