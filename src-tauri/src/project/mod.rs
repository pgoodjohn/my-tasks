use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use tauri::State;
use std::sync::Mutex;
use uuid::Uuid;

pub mod commands;
pub mod detail;
mod test;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub title: String,
    pub emoji: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,
    pub archived_at_utc: Option<DateTime<Utc>>,
}

impl Project {
    pub fn new(title: String, emoji: Option<String>, color: Option<String>, description: Option<String>) -> Self {
        Project {
            id: Uuid::now_v7(),
            title: title,
            emoji: emoji,
            color: color,
            description: description,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
            archived_at_utc: None,
        }
    }

    pub fn save(&mut self, connection: &Connection) -> Result<&Self, ()> {
        if self.exists(connection).unwrap() {
            return self.update(connection);
        }

        connection.execute(
            "INSERT INTO projects (id, title, color, emoji, description, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                &self.id.to_string(), 
                &self.title,
                &self.color,
                &self.emoji,
                &self.description,
                &self.created_at_utc.to_rfc3339(), 
                &self.updated_at_utc.to_rfc3339()],
        ).unwrap();

        Ok(self)
    }

    fn exists(&self, connection: &Connection) -> Result<bool, String> {
        let mut stmt = connection.prepare("SELECT COUNT(*) FROM projects WHERE id = ?1").unwrap();
        let count = stmt.query_row(rusqlite::params![self.id.to_string()], |row| {
            row.get(0)
        });

        match count {
            Ok(0) => Ok(false),
            Ok(_) => Ok(true),
            Err(_) => Err("Failed validating project.".to_string())
        }
    }

    fn update(&mut self, connection: &Connection) -> Result<&Self, ()> {
        self.updated_at_utc = Utc::now();

        connection.execute(
            "UPDATE projects SET title = ?1, emoji = ?2, color = ?3, description = ?4, updated_at_utc = ?5, archived_at_utc = ?6 WHERE id = ?7",
            rusqlite::params![
                &self.title,
                &self.emoji,
                &self.color,
                &self.description,
                &self.updated_at_utc.to_rfc3339(),
                self.archived_at_utc.map(|date| date.to_rfc3339()),
                &self.id.to_string()],
        ).unwrap();

        Ok(self)
    }

    pub fn load_by_id(id: Uuid, connection: &Connection) -> Result<Option<Self>> {
        let mut stmt = connection.prepare("SELECT * FROM projects WHERE id = ?1 LIMIT 1").unwrap();
        let project = stmt.query_row(rusqlite::params![id.to_string()], |row| {
            Project::from_row(row)
        });

        match project {
            Ok(project) => Ok(Some(project)),
            Err(_) => {
                log::error!("Could not find project with ID: {:?}", id);
                Ok(None)
            }
        }

    }

    pub fn list_not_archived_projects(connection: &Connection) -> Result<Vec<Project>, ()> {
        let mut stmt = connection.prepare("SELECT * FROM projects WHERE archived_at_utc IS NULL").unwrap();
        let projects = stmt.query_map([], |row| {
            Project::from_row(row)
        });

        let mut project_list = Vec::new();
        for project in projects.unwrap() {
            project_list.push(project.unwrap());
        }

        Ok(project_list)
    }

    pub fn list_all_projects(connection: &Connection) -> Result<Vec<Project>, ()> {
        let mut stmt = connection.prepare("SELECT * FROM projects").unwrap();
        let projects = stmt.query_map([], |row| {
            Project::from_row(row)
        });

        let mut project_list = Vec::new();
        for project in projects.unwrap() {
            project_list.push(project.unwrap());
        }

        Ok(project_list)
    }

    fn from_row(row: &Row) -> Result<Self> {
        let uuid_string: String = row.get("id").unwrap();
        let created_at_string: String = row.get("created_at_utc").unwrap();
        let updated_at_string: String = row.get("updated_at_utc").unwrap();
        let archived_at_string: Option<String> = row.get("archived_at_utc").unwrap();

        Ok(Project {
            id: Uuid::parse_str(&uuid_string).unwrap(),
            title: row.get("title").unwrap(),
            emoji: row.get("emoji").unwrap(),
            color: row.get("color").unwrap(),
            description: row.get("description").ok(),
            created_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&created_at_string).unwrap()),
            updated_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&updated_at_string).unwrap()),
            archived_at_utc: match archived_at_string {
                Some(s) => Some(DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&s).unwrap())),
                None => None,
            }
        })
    }

    pub fn count_open_tasks_for_project(&self, connection: &Connection) -> Result<i64> {
        let mut stmt = connection.prepare("SELECT COUNT(*) FROM tasks WHERE project_id = ?1 AND completed_at_utc IS NULL").unwrap();
        let count = stmt.query_row(rusqlite::params![self.id.to_string()], |row| {
            row.get(0)
        });

        count
    }
}

#[tauri::command]
pub fn create_project_command(
    title: String,
    emoji: Option<String>,
    color: Option<String>,
    description: Option<String>,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running create project command for: {:?} | {:?}", title, description);
    let mut project = Project::new(
        title,
        emoji,
        color,
        description
    );

    project.save(&db.get().unwrap()).unwrap();

    Ok(serde_json::to_string(&project).unwrap())
}

#[tauri::command]
pub fn archive_project_command(
    project_id: String,
    db: State<Pool<SqliteConnectionManager>>,
    configuration: State<Mutex<crate::configuration::Configuration>>,
) -> Result<String, String> {
    log::debug!("Running archive project command for project ID: {}", project_id);
    let conn = db.get().unwrap(); // Get a connection from the pool
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string()).unwrap();

    let project = Project::load_by_id(uuid, &conn).unwrap();

    let mut locked_configuration = configuration.lock().unwrap();
    locked_configuration.remove_favorite_project(&project_id);

    match project {
        Some(mut project) => {
            project.archived_at_utc = Some(Utc::now());
            project.save(&conn).unwrap();
            return Ok(serde_json::to_string(&project).unwrap());
        }
        None => {
            return Err("Project not found".to_string());
        }
    }
}

#[tauri::command]
pub fn update_project_command(
    project_id: String,
    new_title: Option<String>,
    new_emoji: Option<String>,
    new_color: Option<String>,
    new_description: Option<String>,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running update project command for: {:?} | {:?}", new_title, new_description);
    let conn = db.get().unwrap(); // Get a connection from the pool
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string()).unwrap();

    let project = Project::load_by_id(uuid, &conn).unwrap();

    match project {
        Some(mut project) => {
            project.title = new_title.unwrap_or(project.title);
            project.emoji = new_emoji.or(project.emoji);
            project.description = new_description.or(project.description);
            project.color = new_color.or(project.color);
            project.updated_at_utc = Utc::now();
            project.save(&conn).unwrap();
            return Ok(serde_json::to_string(&project).unwrap());
        }
        None => {
            return Err("Project not found".to_string());
        }
    }
}