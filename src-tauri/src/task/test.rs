#[cfg(test)]
mod test {
    use super::super::commands::TaskManager;
    use super::super::Task;
    use crate::task::CreateTaskData;
    use crate::task::UpdatedTaskData;

    use sqlx::sqlite::SqlitePool;
    use sqlx::Error;

    async fn create_in_memory_pool() -> Result<SqlitePool, Error> {
        let pool = SqlitePool::connect(":memory:").await?;
        Ok(pool)
    }

    async fn apply_migrations(pool: &SqlitePool) -> Result<(), Error> {
        let mut connection = pool.acquire().await.unwrap();

        sqlx::query(
            r#"
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
    )
        "#,
        )
        .execute(&mut *connection)
        .await?;

        sqlx::query(
            r#"
    CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        emoji TEXT,
        color TEXT,
        description TEXT,
        created_at_utc DATETIME NOT NULL,
        updated_at_utc DATETIME NOT NULL,
        archived_at_utc DATETIME
    )
        "#,
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_task_save_and_load() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = TaskManager::new(&db_pool).unwrap();

        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                deadline_at_utc: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        assert_eq!("New Task".to_string(), new_task.title);

        let mut test_connection = db_pool.acquire().await.unwrap();

        let loaded_task = Task::load_by_id(new_task.id, &mut test_connection)
            .await
            .unwrap();

        assert!(loaded_task.is_some());
        assert_eq!(new_task.title, loaded_task.unwrap().title)
    }

    #[tokio::test]
    async fn it_updates_a_task() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = TaskManager::new(&db_pool).unwrap();

        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                deadline_at_utc: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        let update_task_data = UpdatedTaskData {
            title: "Updated Title".to_string(),
            description: None,
            project_id: None,
            due_date: None,
            deadline: None,
        };

        let updated_task = manager
            .update_task(new_task.id, update_task_data)
            .await
            .unwrap();

        assert!(updated_task.is_some());
        assert_eq!("Updated Title".to_string(), updated_task.unwrap().title);
    }

    // #[test]
    // fn test_loads_task_due_today() {
    //     let conn = _setup_in_memory_db();
    //     let title = String::from("Test Task");
    //     let description = Some(String::from("This is a test task."));

    //     let mut task = Task::new(title.clone(), description.clone(), None, None, None);
    //     task.due_at_utc = Some(Utc::now());
    //     task.save(&conn).unwrap();

    //     let tasks = Task::load_due_before(Utc::now(), &conn).unwrap();
    //     assert_eq!(tasks.len(), 1);
    //     assert_eq!(tasks[0].title, title);
    // }

    // #[test]
    // fn test_loads_task_due_yesterday() {
    //     let conn = _setup_in_memory_db();
    //     let title = String::from("Test Task");
    //     let description = Some(String::from("This is a test task."));

    //     let mut task = Task::new(title.clone(), description.clone(), None, None, None);
    //     task.due_at_utc = Some(
    //         Utc::now()
    //             .checked_sub_signed(chrono::Duration::days(1))
    //             .unwrap(),
    //     );
    //     task.save(&conn).unwrap();

    //     let tasks = Task::load_due_before(Utc::now(), &conn).unwrap();
    //     assert_eq!(tasks.len(), 1);
    //     assert_eq!(tasks[0].title, title);
    // }
}
