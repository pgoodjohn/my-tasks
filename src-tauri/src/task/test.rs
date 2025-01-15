#[cfg(test)]
mod test {
    use super::super::Project;
    use super::super::Task;
    use chrono::Utc;
    use rusqlite::Connection;

    fn _setup_in_memory_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            project_id TEXT,
            due_at_utc DATETIME,
            deadline_at_utc DATETIME,
            created_at_utc DATETIME NOT NULL,
            completed_at_utc DATETIME,
            updated_at_utc DATETIME NOT NULL
        );",
            [],
        )
        .unwrap();

        conn.execute(
            "
        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            emoji TEXT,
            color TEXT,
            description TEXT,
            created_at_utc DATETIME NOT NULL,
            updated_at_utc DATETIME NOT NULL,
            archived_at_utc DATETIME
        );",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_task_new() {
        let title = String::from("Test Task");
        let description = Some(String::from("This is a test task."));
        let task = Task::new(title.clone(), description.clone(), None, None, None);
        assert_eq!(task.description, description);
        assert!(task.id.to_string().len() > 0);
        assert!(task.created_at_utc <= Utc::now());
        assert!(task.updated_at_utc <= Utc::now());
        assert!(task.completed_at_utc.is_none());
    }

    #[test]
    fn test_task_save_and_update() {
        let conn = _setup_in_memory_db();
        let title = String::from("Test Task");
        let description = Some(String::from("This is a test task."));
        let project = Project::new(String::from("Test Project"), None, None, None);

        let mut task = Task::new(
            title.clone(),
            description.clone(),
            Some(project),
            None,
            None,
        );
        task.save(&conn).unwrap();

        let saved_task = Task::load_by_id(task.id, &conn).unwrap().unwrap();
        assert_eq!(saved_task.title, title);
        assert_eq!(saved_task.description, description);
        assert_eq!(saved_task.id, task.id);
        assert_eq!(saved_task.created_at_utc, task.created_at_utc);
        assert_eq!(saved_task.updated_at_utc, task.updated_at_utc);
        assert!(saved_task.completed_at_utc.is_none());

        let new_title = String::from("Updated Task");
        task.title = new_title.clone();
        task.save(&conn).unwrap();

        let updated_task = Task::load_by_id(task.id, &conn).unwrap().unwrap();
        assert_eq!(updated_task.title, new_title);
    }

    #[test]
    fn test_loads_task_due_today() {
        let conn = _setup_in_memory_db();
        let title = String::from("Test Task");
        let description = Some(String::from("This is a test task."));

        let mut task = Task::new(title.clone(), description.clone(), None, None, None);
        task.due_at_utc = Some(Utc::now());
        task.save(&conn).unwrap();

        let tasks = Task::load_due_before(Utc::now(), &conn).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, title);
    }

    #[test]
    fn test_loads_task_due_yesterday() {
        let conn = _setup_in_memory_db();
        let title = String::from("Test Task");
        let description = Some(String::from("This is a test task."));

        let mut task = Task::new(title.clone(), description.clone(), None, None, None);
        task.due_at_utc = Some(
            Utc::now()
                .checked_sub_signed(chrono::Duration::days(1))
                .unwrap(),
        );
        task.save(&conn).unwrap();

        let tasks = Task::load_due_before(Utc::now(), &conn).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, title);
    }
}
