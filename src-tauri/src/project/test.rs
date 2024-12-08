#[cfg(test)]
use super::Project;
use chrono::Utc;
use rusqlite::Connection;

fn setup_in_memory_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE projects (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                emoji TEXT,
                color TEXT,
                description TEXT,
                created_at_utc TEXT NOT NULL,
                updated_at_utc TEXT NOT NULL,
                archived_at_utc TEXT
            )",
        [],
    )
    .unwrap();
    conn
}

#[test]
fn test_project_new() {
    let title = String::from("Test Project");
    let description = Some(String::from("This is a test project."));
    let project = Project::new(title.clone(), None, None, description.clone());
    assert_eq!(project.description, description);
    assert!(project.id.to_string().len() > 0);
    assert!(project.created_at_utc <= Utc::now());
    assert!(project.updated_at_utc <= Utc::now());
    assert!(project.archived_at_utc.is_none());
}

#[test]
fn test_project_save() {
    let conn = setup_in_memory_db();
    let title = String::from("Test Project");
    let description = Some(String::from("This is a test project."));

    let mut project = Project::new(title.clone(), None, None, description.clone());
    project.save(&conn).unwrap();

    let saved_project = Project::load_by_id(project.id, &conn).unwrap().unwrap();
    assert_eq!(saved_project.title, title);
    assert_eq!(saved_project.description, description);
    assert_eq!(saved_project.id, project.id);
    assert_eq!(saved_project.created_at_utc, project.created_at_utc);
    assert_eq!(saved_project.updated_at_utc, project.updated_at_utc);
    assert!(saved_project.archived_at_utc.is_none());
}

#[test]
fn test_archiving_a_project_still_allows_you_to_load_it() {
    let conn = setup_in_memory_db();
    let title = String::from("Test Project");
    let description = Some(String::from("This is a test project."));
    let mut project = Project::new(title.clone(), None, None, description.clone());
    project.save(&conn).unwrap();

    let saved_project = Project::load_by_id(project.id, &conn).unwrap().unwrap();
    assert_eq!(saved_project.title, title);
    assert_eq!(saved_project.description, description);
    assert_eq!(saved_project.id, project.id);
    assert_eq!(saved_project.created_at_utc, project.created_at_utc);
    assert_eq!(saved_project.updated_at_utc, project.updated_at_utc);
    assert!(saved_project.archived_at_utc.is_none());

    project.archived_at_utc = Some(Utc::now());
    project.save(&conn).unwrap();

    let archived_project = Project::load_by_id(project.id, &conn).unwrap().unwrap();
    assert_eq!(archived_project.title, title);
    assert_eq!(archived_project.description, description);
    assert_eq!(archived_project.id, project.id);
    assert_eq!(archived_project.created_at_utc, project.created_at_utc);
    assert_eq!(archived_project.updated_at_utc, project.updated_at_utc);
    assert!(archived_project.archived_at_utc.is_some());
}
