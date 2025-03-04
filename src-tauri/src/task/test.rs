#[cfg(test)]
mod task_tests {
    use super::super::manager::TaskManager;
    use crate::task::repository::{RepositoryProvider, TaskRepository};
    use crate::task::{CreateTaskData, UpdatedTaskData};

    use sqlx::migrate::MigrateDatabase;
    use sqlx::sqlite::SqlitePool;
    use sqlx::Sqlite;

    async fn setup_test_db() -> Result<RepositoryProvider, sqlx::Error> {
        let url = format!("sqlite://{}", ":memory:");

        if !Sqlite::database_exists(&url).await.unwrap_or(false) {
            Sqlite::create_database(&url).await?;
        }

        let pool = SqlitePool::connect(&url).await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(RepositoryProvider::new(pool))
    }

    #[tokio::test]
    async fn test_task_save_and_load() {
        let provider = setup_test_db().await.unwrap();
        let manager = TaskManager::new(&provider);

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

        let mut repository = provider.task_repository().await.unwrap();
        let loaded_task = repository.find_by_id(new_task.id).await.unwrap();

        assert!(loaded_task.is_some());
        assert_eq!(new_task.title, loaded_task.unwrap().title);
    }

    #[tokio::test]
    async fn it_updates_a_task() {
        let provider = setup_test_db().await.unwrap();
        let manager = TaskManager::new(&provider);

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
        let provider = setup_test_db().await.unwrap();
        let manager = TaskManager::new(&provider);

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
        let provider = setup_test_db().await.unwrap();
        let manager = TaskManager::new(&provider);

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
        let provider = setup_test_db().await.unwrap();
        let manager = TaskManager::new(&provider);

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
