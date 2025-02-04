use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use crate::task::manager::TaskManager;

#[tauri::command]
pub async fn tick_task_command(
    task_id: String,
    db: State<'_, SqlitePool>,
) -> Result<String, String> {
    log::debug!("Running tick task command for card ID: {}", task_id);
    let uuid = Uuid::parse_str(&task_id).map_err(|e| e.to_string())?;

    let manager = TaskManager::new(&db).unwrap();

    manager.tick(uuid).await.unwrap();

    Ok("{}".to_string())
}
