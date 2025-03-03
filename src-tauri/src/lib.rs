#[macro_use]
extern crate dotenvy_macro;

use sqlx::sqlite::SqlitePool;
use tauri::async_runtime::Mutex;
use tauri::Manager;

mod chart;
mod configuration;
mod errors;
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

            // Register Sentry
            let error_handler = errors::ErrorHandler::initialise(detect_mode());
            app.manage(error_handler);

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
            chart::tauri::queries::load_rolling_week_day_charts_command,
            chart::tauri::queries::load_project_activity_stats_command,
            configuration::commands::load_configuration_command,
            project::tauri::actions::archive_project_command,
            project::tauri::actions::create_project_command,
            project::tauri::actions::update_project_command,
            project::tauri::queries::load_projects_command,
            project::tauri::queries::count_open_tasks_for_project_command,
            project::tauri::queries::load_project_details_command,
            project::tauri::queries::add_favorite_project_command,
            project::tauri::queries::remove_favorite_project_command,
            project::tauri::queries::load_favorite_projects_command,
            task::tauri::actions::create_task_command,
            task::tauri::actions::update_task_command,
            task::tauri::actions::delete_task_command,
            task::tauri::actions::create_subtask_for_task_command,
            task::tauri::actions::complete_task_command,
            task::tauri::actions::promote_task_to_project_command,
            task::tauri::queries::load_subtasks_for_task_command,
            task::tauri::queries::load_task_by_id_command,
            task::tauri::queries::load_task_activity_statistics_command,
            task::tauri::queries::load_tasks_due_today_command,
            task::tauri::queries::load_tasks_inbox_command,
            task::tauri::queries::load_tasks_command,
            task::tauri::queries::load_completed_tasks_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
