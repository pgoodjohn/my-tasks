use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, Row};
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub project: Option<Project>,
    pub due_at_utc: Option<DateTime<Utc>>,
    pub created_at_utc: DateTime<Utc>,
    pub completed_at_utc: Option<DateTime<Utc>>,
    pub updated_at_utc: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, description: Option<String>, project: Option<Project>, due_at_utc: Option<DateTime<Utc>>) -> Self {
        Task {
            id: Uuid::now_v7(),
            title: title,
            description: description,
            project: project,
            due_at_utc: due_at_utc,
            created_at_utc: Utc::now(),
            completed_at_utc: None, 
            updated_at_utc: Utc::now(),
        }
    }

    fn is_stored(&self) -> bool {
        false
    }

    pub fn save(&self, connection: &Connection) -> Result<&Self, ()> {
        if self.is_stored() {
            return Ok(self);
        }

        connection.execute(
            "INSERT INTO tasks (id, title, description, project_id, due_at_utc, created_at_utc, updated_at_utc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                &self.id.to_string(), 
                &self.title,
                &self.description,
                &self.project.as_ref().map(|project| project.id.to_string()),
                &self.due_at_utc.map(|date| date.to_rfc3339()),
                &self.created_at_utc.to_rfc3339(), 
                &self.updated_at_utc.to_rfc3339()],
        ).unwrap();

        Ok(self)
    }

    fn from_row(row: &Row, connection: &Connection) -> Result<Self> {
        let uuid_string: String = row.get("id").unwrap();
        let project_uuid_string: Option<String> = row.get("project_id").ok();
        let created_at_string: String = row.get("created_at_utc").unwrap();
        let updated_at_string: String = row.get("updated_at_utc").unwrap();
        let completed_at_string: Option<String> = row.get("completed_at_utc").ok();

        Ok(Task {
            id: Uuid::parse_str(&uuid_string).unwrap(),
            title: row.get("title").unwrap(),
            description: row.get("description").ok(),
            project: match project_uuid_string {
                Some(uuid) => Project::load_by_id(Uuid::parse_str(&uuid).unwrap(), &connection).unwrap(),
                None => None
            },
            due_at_utc: row.get("due_at_utc").ok().map(|date: String| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap())),
            created_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&created_at_string).unwrap()),
            completed_at_utc: completed_at_string.map(|s| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&s).unwrap())),
            updated_at_utc: DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&updated_at_string).unwrap())
        })
    }

    pub fn load_for_project(project_id: Uuid, connection: &Connection) -> Result<Vec<Self>> {
        let mut stmt = connection.prepare("SELECT * FROM tasks WHERE project_id = ?1 ORDER BY created_at_utc DESC").unwrap();
        let task_iter = stmt.query_map(rusqlite::params![project_id.to_string()], |row| {
            Task::from_row(row, connection)
        }).unwrap();

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task.unwrap());
        }

        Ok(tasks)
    }
}

#[tauri::command]
pub fn save_task_command(
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running save task command for: {:?} | {:?} | {:?}", title, description, due_date);

    let connection = db.get().unwrap();

    let task = Task::new(
        title,
        description,
        match project_id {
            Some(id) => {
                let project = Project::load_by_id(Uuid::parse_str(&id).unwrap(), &connection).unwrap();
                match project {
                    Some(project) => Some(project),
                    None => {
                        return Err("Could not find project with id".to_string());
                    }
                }
            },
            None => None
        },
        match due_date {
            Some(date) => Some(DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap())),
            None => None
        }
    );

    task.save(&connection).unwrap();

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub fn load_tasks_command(
    include_completed: bool,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running load tasks command - include_completed: {:?}", include_completed);

    let conn = db.get().unwrap(); // Get a connection from the pool
    let query = if include_completed {
        "SELECT * FROM tasks WHERE created_at_utc IS NOT NULL ORDER BY created_at_utc DESC"
    } else {
        "SELECT * FROM tasks WHERE created_at_utc IS NOT NULL AND completed_at_utc IS NULL ORDER BY created_at_utc DESC"
    };
    let mut stmt = conn.prepare(query).unwrap(); // Prepare the SQL statement
    let task_iter = stmt.query_map([], |row| {
        Task::from_row(row, &conn) // Map each row to a Card object
    }).unwrap();

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task.unwrap()); // Collect all cards into a vector
    }

    Ok(serde_json::to_string(&tasks).unwrap())
}

#[tauri::command]
pub fn delete_task_command(
    task_id: String,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running delete task command for card ID: {}", task_id);
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&task_id).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM tasks WHERE id = ?1",
        rusqlite::params![&uuid.to_string()],
    ).map_err(|e| e.to_string())?;

    Ok(format!("Task with ID {} deleted successfully", &task_id))
}

#[tauri::command]
pub fn complete_task_command(
    task_id: String,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running complete task command for card ID: {}", task_id);
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&task_id).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE tasks SET completed_at_utc = ?1 WHERE id = ?2",
        rusqlite::params![Utc::now().to_rfc3339(), &uuid.to_string()],
    ).map_err(|e| e.to_string())?;

    Ok(format!("Card with ID {} completed successfully", task_id))
}

#[tauri::command]
pub fn update_task_command(
    task_id: String,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    project_id: Option<String>,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running update task command for: {:?} | {:?} | {:?}", title, description, due_date);
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&task_id).map_err(|e| e.to_string()).unwrap();

    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1 LIMIT 1").unwrap();
    let mut task = stmt.query_row(rusqlite::params![uuid.to_string()], |row| {
        Task::from_row(row, &conn)
    }).map_err(|e| e.to_string()).unwrap();

    task.title = title;
    task.description = description;
    task.due_at_utc = due_date.map(|date| DateTime::<Utc>::from(DateTime::parse_from_rfc3339(&date).unwrap()));
    task.updated_at_utc = Utc::now();

    match project_id {
        Some(project_id) => {
            let project_uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string()).unwrap();
            task.project = Project::load_by_id(project_uuid, &conn).unwrap();
        },
        None => {
            task.project = None;
        }
    }

    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2, due_at_utc = ?3, updated_at_utc = ?4, project_id = ?5 WHERE id = ?6",
        rusqlite::params![&task.title, &task.description, task.due_at_utc.map(|date| date.to_rfc3339()), task.updated_at_utc.to_rfc3339(), task.project.as_ref().map(|project| project.id.to_string()), &uuid.to_string()],
    ).map_err(|e| e.to_string())?;

    Ok(serde_json::to_string(&task).unwrap())
}

#[tauri::command]
pub fn count_open_tasks_for_project_command(
    project_id: String,
    db: State<Pool<SqliteConnectionManager>>,
) -> Result<String, String> {
    log::debug!("Running count open tasks for project command for project ID: {}", project_id);
    let conn = db.get().unwrap(); // Get a connection from the pool

    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string()).unwrap();

    let project = Project::load_by_id(uuid, &conn).unwrap();

    match project {
        Some(project) => {
            let count = project.count_open_tasks_for_project(&conn).unwrap();
            return Ok(count.to_string());
        }
        None => {
            return Err("Project not found".to_string());
        }
    }
}