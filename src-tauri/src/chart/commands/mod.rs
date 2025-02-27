use chrono::Utc;
use sqlx::SqlitePool;

use tauri::State;

use crate::chart::manager::ChartManager;
use crate::errors::handle_error;

#[tauri::command]
pub async fn load_rolling_week_day_charts_command(
    db_pool: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running load day charts command");

    let manager = ChartManager::new(&db_pool);

    let day_charts = manager
        .load_rolling_week_day_charts(Utc::now())
        .await
        .map_err(|e| handle_error(&*e))?;

    Ok(serde_json::to_string(&day_charts).unwrap())
}
