use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use tauri::State;

use crate::chart::manager::ChartManager;
use crate::errors::handle_error;

#[tauri::command]
pub async fn load_rolling_week_day_charts_command(
    since: DateTime<Utc>,
    until: DateTime<Utc>,
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load day charts command");

    let manager = ChartManager::new(&db_pool);

    let day_charts = manager
        .load_rolling_week_day_charts(since, until)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&day_charts).unwrap())
}

#[tauri::command]
pub async fn load_project_activity_stats_command(
    since: DateTime<Utc>,
    until: DateTime<Utc>,
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load project activity stats command");

    let manager = ChartManager::new(&db_pool);

    let stats = manager
        .load_project_activity_stats(since, until)
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&stats).unwrap())
}
