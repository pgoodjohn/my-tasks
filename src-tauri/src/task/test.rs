#[cfg(test)]
mod task_tests {
    use super::super::manager::TaskManager;
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
        parent_task_id TEXT,
        due_at_utc DATETIME,
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

        let manager = TaskManager::new(&db_pool);

        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
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

        let manager = TaskManager::new(&db_pool);

        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        let update_task_data = UpdatedTaskData {
            title: "Updated Title".to_string(),
            description: None,
            project_id: None,
            due_date: None,
        };

        let updated_task = manager
            .update_task(new_task.id, update_task_data)
            .await
            .unwrap();

        assert!(updated_task.is_some());
        assert_eq!("Updated Title".to_string(), updated_task.unwrap().title);
    }

    #[tokio::test]
    async fn it_creates_a_task_and_a_subtask_for_it() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = TaskManager::new(&db_pool);

        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        let new_task_id = new_task.id.clone();

        let subtask = manager
            .create_subtask_for_task(
                new_task,
                CreateTaskData {
                    title: "New Task".to_string(),
                    description: None,
                    project_id: None,
                    due_at_utc: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(new_task_id, subtask.parent_task_id.unwrap());

        let new_task_subtasks = manager.load_subtasks_for_task(new_task_id).await.unwrap();

        assert_eq!(1, new_task_subtasks.len());
    }

    #[tokio::test]
    async fn it_completes_a_task() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = TaskManager::new(&db_pool);
        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        manager.complete_task(new_task.id).await.unwrap();

        let loaded_task = manager.load_by_id(new_task.id).await.unwrap().unwrap();
        assert!(loaded_task.completed_at_utc.is_some());
    }

    #[tokio::test]
    async fn completing_a_task_also_completes_all_its_subtasks() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = TaskManager::new(&db_pool);
        let new_task = manager
            .create_task(CreateTaskData {
                title: "New Task".to_string(),
                description: None,
                project_id: None,
                due_at_utc: None,
            })
            .await
            .unwrap();

        let new_task_uuid = new_task.id.clone();

        let subtask = manager
            .create_subtask_for_task(
                new_task,
                CreateTaskData {
                    title: "New Task".to_string(),
                    description: None,
                    project_id: None,
                    due_at_utc: None,
                },
            )
            .await
            .unwrap();

        manager.complete_task(new_task_uuid).await.unwrap();

        let reloaded_subtask = manager.load_by_id(subtask.id).await.unwrap().unwrap();

        assert!(reloaded_subtask.completed_at_utc.is_some());
    }
}
