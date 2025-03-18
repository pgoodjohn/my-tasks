#[cfg(test)]
mod manager_test {
    use crate::project::manager::ProjectsManager;
    use crate::repository::RepositoryProvider;

    use chrono::Utc;
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
    async fn it_creates_a_project() {
        let title = String::from("Test Project");

        let provider = setup_test_db().await.unwrap();
        let mut project_repository = provider.project_repository().await.unwrap();
        let mut task_repository = provider.task_repository().await.unwrap();
        let mut project_manager =
            ProjectsManager::new(&mut project_repository, &mut task_repository);

        let project = project_manager
            .create_project(title, None, None, None)
            .await
            .unwrap();

        assert_eq!(project.title, "Test Project");
        assert!(project.description.is_none());
        assert!(project.emoji.is_none());
        assert!(project.color.is_none());
        assert!(project.id.to_string().len() > 0);
        assert!(project.created_at_utc <= Utc::now());
        assert!(project.updated_at_utc <= Utc::now());
        assert!(project.archived_at_utc.is_none());
    }

    #[tokio::test]
    async fn it_updates_a_project() {
        let provider = setup_test_db().await.unwrap();
        let mut project_repository = provider.project_repository().await.unwrap();
        let mut task_repository = provider.task_repository().await.unwrap();
        let mut project_manager =
            ProjectsManager::new(&mut project_repository, &mut task_repository);

        let project = project_manager
            .create_project("Test Project".to_string(), None, None, None)
            .await
            .unwrap();

        let updated_project = project_manager
            .update_project(
                project.id,
                "A new Title".to_string(),
                Some("🧪".to_string()),
                None,
                None,
            )
            .await
            .unwrap();

        assert_eq!(updated_project.title, "A new Title");
        assert_eq!(updated_project.emoji, Some("🧪".to_string()));
        assert!(updated_project.color.is_none());
        assert!(updated_project.description.is_none());

        assert_ne!(
            updated_project.updated_at_utc,
            updated_project.created_at_utc
        );
        assert_ne!(updated_project.updated_at_utc, project.updated_at_utc);
    }

    #[tokio::test]
    async fn it_archives_a_project() {
        let provider = setup_test_db().await.unwrap();
        let mut project_repository = provider.project_repository().await.unwrap();
        let mut task_repository = provider.task_repository().await.unwrap();
        let mut project_manager =
            ProjectsManager::new(&mut project_repository, &mut task_repository);

        let project = project_manager
            .create_project("Test Project".to_string(), None, None, None)
            .await
            .unwrap();

        let archived_project = project_manager.archive_project(project.id).await.unwrap();

        assert!(archived_project.archived_at_utc.is_some());
    }

    #[tokio::test]
    async fn it_favorites_and_unfavorites_a_project() {
        let provider = setup_test_db().await.unwrap();
        let mut project_repository = provider.project_repository().await.unwrap();
        let mut task_repository = provider.task_repository().await.unwrap();
        let mut project_manager =
            ProjectsManager::new(&mut project_repository, &mut task_repository);

        let project = project_manager
            .create_project("Test Project".to_string(), None, None, None)
            .await
            .unwrap();

        assert!(!project.is_favorite);

        let favorite_project = project_manager.add_favorite(project.id).await.unwrap();

        assert_eq!(project.id, favorite_project.id);
        assert!(favorite_project.is_favorite);

        let unfavorited_project = project_manager
            .remove_favorite(favorite_project.id)
            .await
            .unwrap();

        assert_eq!(favorite_project.id, unfavorited_project.id);
        assert!(!unfavorited_project.is_favorite);
    }
}
