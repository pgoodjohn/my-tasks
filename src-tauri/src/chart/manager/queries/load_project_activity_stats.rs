use chrono::{DateTime, Utc};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite};
use std::error::Error;

use crate::chart::manager::queries::ProjectActivityStats;
use crate::chart::manager::ChartManager;

impl ProjectActivityStats {
    pub async fn load_for_project(
        project_id: String,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, Box<dyn Error>> {
        let project_row = sqlx::query("SELECT id, title FROM projects WHERE id = ?1")
            .bind(&project_id)
            .fetch_one(&mut **connection)
            .await?;

        let project_id = project_row.get::<String, _>("id");
        let project_title = project_row.get::<String, _>("title");

        let completed_count = sqlx::query(
            "SELECT COUNT(*) as count FROM tasks WHERE project_id = ?1 AND completed_at_utc IS NOT NULL AND completed_at_utc >= ?2 AND completed_at_utc <= ?3"
        )
        .bind(&project_id)
        .bind(since.to_rfc3339())
        .bind(until.to_rfc3339())
        .fetch_one(&mut **connection)
        .await?
        .get::<i32, _>("count");

        let created_count = sqlx::query(
            "SELECT COUNT(*) as count FROM tasks WHERE project_id = ?1 AND created_at_utc >= ?2 AND created_at_utc <= ?3",
        )
        .bind(&project_id)
        .bind(since.to_rfc3339())
        .bind(until.to_rfc3339())
        .fetch_one(&mut **connection)
        .await?
        .get::<i32, _>("count");

        Ok(ProjectActivityStats {
            project_id,
            project_title,
            completed_tasks: completed_count,
            created_tasks: created_count,
        })
    }
}

impl ChartManager<'_> {
    pub async fn load_project_activity_stats(
        &self,
        since: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> Result<Vec<ProjectActivityStats>, Box<dyn std::error::Error>> {
        let mut connection = self.db_pool.acquire().await?;

        // Get only projects that have either completed or created tasks in the date range
        let project_ids = sqlx::query(
            "SELECT DISTINCT p.id 
            FROM projects p 
            LEFT JOIN tasks t ON p.id = t.project_id 
            WHERE p.archived_at_utc IS NULL 
            AND (
                (t.completed_at_utc IS NOT NULL AND t.completed_at_utc >= ?1 AND t.completed_at_utc <= ?2)
                OR (t.created_at_utc >= ?1 AND t.created_at_utc <= ?2)
            )",
        )
        .bind(since.to_rfc3339())
        .bind(until.to_rfc3339())
        .fetch_all(&mut *connection)
        .await?;

        let mut stats = vec![];

        for row in project_ids {
            let project_id = row.get::<String, _>("id");
            let project_stats =
                ProjectActivityStats::load_for_project(project_id, since, until, &mut connection)
                    .await?;
            stats.push(project_stats);
        }

        Ok(stats)
    }
}
