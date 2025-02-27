use std::collections::HashMap;

use sqlx::{pool::PoolConnection, Row as SqlxRow, Sqlite, SqlitePool};
use tauri::State;
use uuid::Uuid;

use super::{CreateTaskData, UpdatedTaskData};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::error::Error;

use crate::errors::handle_error;

use super::manager::TaskManager;

#[derive(Serialize)]
pub struct PeriodTaskStatistic(HashMap<String, DateTaskStatistic>);

impl PeriodTaskStatistic {
    pub async fn load(
        connection: &mut PoolConnection<Sqlite>,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut statistics = vec![];

        let sqlx_result = sqlx::query(
           "SELECT COUNT(*) as count, strftime('%Y-%m-%d', completed_at_utc) as date FROM tasks WHERE completed_at_utc IS NOT NULL GROUP BY date ORDER BY date DESC",
        )
        .fetch_all(&mut **connection)
        .await?;

        for row in sqlx_result {
            let date = row.get("date");
            let level = match row.get("count") {
                0 => 0,
                1..=3 => 1,
                4..=6 => 2,
                7..=9 => 3,
                _ => 4,
            };
            let date_statistic = DateTaskStatistic {
                level,
                data: DateTaskStatisticData {
                    completed_tasks: row.get("count"),
                },
            };

            let mut period_statistic =
                PeriodTaskStatistic(HashMap::<String, DateTaskStatistic>::new());
            period_statistic.0.insert(date, date_statistic);

            statistics.push(period_statistic)
        }

        Ok(statistics)
    }
}

#[derive(Serialize)]
pub struct DateTaskStatistic {
    level: i64,
    data: DateTaskStatisticData,
}

#[derive(Serialize)]
pub struct DateTaskStatisticData {
    completed_tasks: i64,
}
