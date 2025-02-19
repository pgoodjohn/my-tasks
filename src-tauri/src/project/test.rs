#[cfg(test)]
mod manager_test {

    use crate::project::commands::ProjectsManager;

    use chrono::Utc;
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
        archived_at_utc DATETIME,
        is_favorite BOOLEAN DEFAULT FALSE
    )
        "#,
        )
        .execute(&mut *connection)
        .await?;

        Ok(())
    }

    #[tokio::test]
    async fn it_creates_a_project() {
        let title = String::from("Test Project");
        let description = Some(String::from("This is a test project."));

        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let project_manager = ProjectsManager::new(&db_pool).unwrap();

        let project = project_manager
            .create_project(title, None, None, description)
            .await
            .unwrap();

        assert_eq!(project.title, "Test Project");
        assert_eq!(
            project.description,
            Some("This is a test project.".to_string())
        );
        assert!(project.id.to_string().len() > 0);
        assert!(project.created_at_utc <= Utc::now());
        assert!(project.updated_at_utc <= Utc::now());
        assert!(project.archived_at_utc.is_none());
    }

    #[tokio::test]
    async fn it_updates_a_project() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let project_manager = ProjectsManager::new(&db_pool).unwrap();

        let project = project_manager
            .create_project(
                "Test Project".to_string(),
                None,
                None,
                Some("This is a test project.".to_string()),
            )
            .await
            .unwrap();

        let updated_project = project_manager
            .update_project(
                project.id,
                Some("A new Title".to_string()),
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
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();
        let project_manager = ProjectsManager::new(&db_pool).unwrap();

        let project = project_manager
            .create_project(
                "Test Project".to_string(),
                None,
                Some("This is a test project.".to_string()),
                None,
            )
            .await
            .unwrap();
        let archived_project = project_manager.archive_project(project.id).await.unwrap();

        assert!(archived_project.archived_at_utc.is_some());
    }

    #[tokio::test]
    async fn it_favorites_and_unfavorites_a_project() {
        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();
        let project_manager = ProjectsManager::new(&db_pool).unwrap();

        let project = project_manager
            .create_project(
                "Test Project".to_string(),
                None,
                Some("This is a test project.".to_string()),
                None,
            )
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
