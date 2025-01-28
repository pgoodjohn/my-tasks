use serde::Serialize;

use chrono::{DateTime, Days, Utc};
use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite, SqlitePool};

use tauri::State;

mod test;

#[tauri::command]
pub async fn load_rolling_week_day_charts_command(
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load day charts command");

    let manager = ChartManager::new(&db_pool).unwrap();

    let day_charts = manager
        .load_rolling_week_day_charts(Utc::now())
        .await
        .unwrap();

    Ok(serde_json::to_string(&day_charts).unwrap())
}

pub struct ChartManager<'a> {
    db_pool: &'a SqlitePool,
}

impl<'a> ChartManager<'a> {
    fn new(db_pool: &'a SqlitePool) -> Result<Self, ()> {
        Ok(ChartManager { db_pool })
    }

    pub async fn load_rolling_week_day_charts(
        &self,
        until: DateTime<Utc>,
    ) -> Result<Vec<RollingWeekDayCharts>, ()> {
        let mut connection = self.db_pool.acquire().await.unwrap();

        let mut charts = vec![];

        for i in 0..7 {
            let query_date = until.checked_sub_days(Days::new(i)).unwrap();
            let charts_data = RollingWeekDayCharts::load_for_date(query_date, &mut connection)
                .await
                .unwrap();
            charts.push(charts_data)
        }

        charts.reverse(); // TODO: Silly hack

        Ok(charts)
    }
}

#[derive(Debug, Serialize)]
pub struct RollingWeekDayCharts {
    pub day: String,
    pub completed_tasks: i32,
    pub created_tasks: i32,
}

impl RollingWeekDayCharts {
    async fn count_completed_tasks_on_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<i32, ()> {
        let mut sqlx_result = sqlx::query(
            "SELECT COUNT(*) AS completed_count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc LIKE ?1 GROUP BY date LIMIT 1"
        )
        .bind(format!("{}%", date.format("%Y-%m-%d")))
        .fetch_all(&mut **connection)
        .await.unwrap();

        match sqlx_result.pop() {
            None => Ok(0),
            Some(r) => Ok(r.get("completed_count")),
        }
    }

    async fn count_created_tasks_on_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<i32, ()> {
        let mut sqlx_result = sqlx::query(
            "SELECT COUNT(*) AS created_count, strftime('%Y-%m-%d', created_at_utc) as date FROM tasks WHERE created_at_utc LIKE ?1 GROUP BY date LIMIT 1"
        )
        .bind(format!("{}%", date.format("%Y-%m-%d")))
        .fetch_all(&mut **connection)
        .await.unwrap();

        match sqlx_result.pop() {
            None => Ok(0),
            Some(r) => Ok(r.get("created_count")),
        }
    }

    pub async fn load_for_date(
        date: DateTime<Utc>,
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Self, ()> {
        Ok(RollingWeekDayCharts {
            day: String::from(format!("{}", date.format("%A"))),
            completed_tasks: RollingWeekDayCharts::count_completed_tasks_on_date(date, connection)
                .await
                .unwrap(),
            created_tasks: RollingWeekDayCharts::count_created_tasks_on_date(date, connection)
                .await
                .unwrap(),
        })
    }
}
