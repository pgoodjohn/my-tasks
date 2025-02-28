use chrono::{DateTime, Days, Utc};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite};
use std::error::Error;

use crate::chart::manager::queries::RollingWeekDayCharts;
use crate::chart::manager::ChartManager;

impl RollingWeekDayCharts {
    async fn count_completed_tasks_on_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<i32, Box<dyn Error>> {
        let mut sqlx_result = sqlx::query(
            "SELECT COUNT(*) AS completed_count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc LIKE ?1 GROUP BY date LIMIT 1"
        )
        .bind(format!("{}%", date.format("%Y-%m-%d")))
        .fetch_all(&mut **connection)
        .await?;

        match sqlx_result.pop() {
            None => Ok(0),
            Some(r) => Ok(r.get("completed_count")),
        }
    }

    async fn count_created_tasks_on_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<i32, Box<dyn Error>> {
        let mut sqlx_result = sqlx::query(
            "SELECT COUNT(*) AS created_count, strftime('%Y-%m-%d', created_at_utc) as date FROM tasks WHERE created_at_utc LIKE ?1 GROUP BY date LIMIT 1"
        )
        .bind(format!("{}%", date.format("%Y-%m-%d")))
        .fetch_all(&mut **connection)
        .await?;

        match sqlx_result.pop() {
            None => Ok(0),
            Some(r) => Ok(r.get("created_count")),
        }
    }

    pub async fn load_for_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, Box<dyn Error>> {
        let completed_tasks =
            RollingWeekDayCharts::count_completed_tasks_on_date(date, connection).await?;

        let created_tasks =
            RollingWeekDayCharts::count_created_tasks_on_date(date, connection).await?;

        Ok(RollingWeekDayCharts {
            day: format!("{}", date.format("%A")),
            completed_tasks,
            created_tasks,
        })
    }
}

impl ChartManager<'_> {
    pub async fn load_rolling_week_day_charts(
        &self,
        until: DateTime<Utc>,
    ) -> Result<Vec<RollingWeekDayCharts>, Box<dyn std::error::Error>> {
        let mut connection = self.db_pool.acquire().await?;

        let mut charts = vec![];

        for i in 0..7 {
            let query_date = until
                .checked_sub_days(Days::new(i))
                .ok_or("Unable to calculate date")?;
            let charts_data =
                RollingWeekDayCharts::load_for_date(query_date, &mut connection).await?;

            charts.push(charts_data)
        }

        charts.reverse(); // TODO: Silly hack

        Ok(charts)
    }
}
