#[cfg(test)]
mod chart_manager_test {
    use super::super::ChartManager;
    use super::super::RollingWeekDayCharts;
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
    async fn it_creates_a_week_worth_of_stats() {
        let expected = vec![
            RollingWeekDayCharts {
                day: String::from("Monday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Sunday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Saturday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Friday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Thursday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Wednesday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
            RollingWeekDayCharts {
                day: String::from("Thursday"),
                completed_tasks: 0,
                created_tasks: 0,
            },
        ];

        let db_pool = create_in_memory_pool().await.unwrap();
        apply_migrations(&db_pool).await.unwrap();

        let manager = ChartManager::new(&db_pool).unwrap();

        let until = Utc::now(); // TODO: make fixd date

        let actual = manager.load_rolling_week_day_charts(until).await.unwrap();

        assert_eq!(expected.len(), actual.len());
    }
}
