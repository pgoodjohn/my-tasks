use tauri::async_runtime::Mutex;
use tauri::State;

use crate::configuration::Configuration;

#[tauri::command]
pub fn load_configuration_command(
    configuration: State<Mutex<Configuration>>,
) -> Result<String, String> {
    log::debug!("Running load_configuration_command. {:?}", configuration);

    let config = configuration.try_lock().unwrap();

    Ok(serde_json::to_string(&*config).unwrap())
}
