use sqlx::sqlite::SqlitePool;
use tauri::async_runtime::Mutex;
use tauri::Manager;

mod chart;
mod commands;
mod configuration;
mod project;
mod storage;
mod task;

use configuration::manager::{ConfigurationManager, ConfigurationMode};

fn detect_mode() -> ConfigurationMode {
    if cfg!(debug_assertions) {
        ConfigurationMode::Development
    } else {
        ConfigurationMode::Desktop
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            plogger::init(cfg!(debug_assertions)); // TODO: Stop using plogger

            let configuration_manager = ConfigurationManager::init(detect_mode()); // TODO: Mode detection

            log::info!("Starting My Tasks!");
            log::debug!(
                "Initialising app with configuration: {:?}",
                configuration_manager.configuration
            );
            // Create a new Tokio runtime
            let rt = tokio::runtime::Runtime::new().unwrap();

            // Use the runtime to block on the async connection
            let db_pool = rt.block_on(async move {
                log::debug!("Setting up db connection pool");

                let configuration_manager = ConfigurationManager::load(detect_mode()).unwrap();

                let db_pool = SqlitePool::connect(
                    configuration_manager
                        .configuration
                        .db_path
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap();
                sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();
                log::debug!("Migrations run successfully");

                db_pool
            });
            app.manage(db_pool.clone());

            app.manage(Mutex::new(configuration_manager.configuration));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            chart::commands::load_rolling_week_day_charts_command,
            configuration::commands::load_configuration_command,
            project::commands::archive_project_command,
            project::commands::create_project_command,
            project::commands::load_projects_command,
            project::commands::update_project_command,
            project::commands::count_open_tasks_for_project_command,
            project::commands::load_project_details_command,
            project::commands::add_favorite_project_command,
            project::commands::remove_favorite_project_command,
            project::commands::load_favorite_projects_command,
            task::commands::tick_task_command,
            task::commands::create_subtask_for_task_command,
            task::commands::load_subtasks_for_task_command,
            task::commands::load_task_by_id_command,
            task::commands::load_task_activity_statistics_command,
            task::commands::load_tasks_due_today_command,
            task::commands::load_tasks_with_deadline_command,
            task::commands::load_tasks_inbox_command,
            task::commands::load_tasks_command,
            task::commands::complete_task_command,
            task::commands::create_task_command,
            task::commands::update_task_command,
            task::commands::delete_task_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
