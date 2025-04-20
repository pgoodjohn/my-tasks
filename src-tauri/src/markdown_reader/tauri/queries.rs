use crate::configuration::manager::ConfigurationManager;
use crate::errors::handle_error;
use crate::markdown_reader::reader;
use tauri::State;

#[tauri::command]
pub async fn load_markdown_tasks_command(
    config_manager: State<'_, ConfigurationManager>,
) -> Result<String, String> {
    log::debug!("Entering load_markdown_tasks_command");

    let config = config_manager.configuration.clone(); // Clone to avoid holding lock

    let folder_path = match config.markdown_tasks_folder {
        Some(ref path) => path.clone(),
        None => {
            log::warn!("Markdown tasks folder not configured.");
            // Return empty list or specific error message if folder isn't set
            return Ok("[]".to_string());
        }
    };

    log::info!("Reading markdown tasks from: {:?}", folder_path);

    match reader::read_markdown_tasks(&folder_path) {
        Ok(tasks) => {
            log::debug!("Successfully read {} markdown tasks", tasks.len());
            serde_json::to_string(&tasks).map_err(|e| handle_error(&e))
        }
        Err(e) => {
            log::error!("Error reading markdown tasks: {}", e);
            Err(handle_error(&e))
        }
    }
}
