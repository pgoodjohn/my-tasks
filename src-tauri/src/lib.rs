#[macro_use]
extern crate dotenvy_macro;

use sqlx::sqlite::SqlitePool;
use tauri::async_runtime::Mutex as AsyncMutex;
use tauri::Manager;
use thiserror::Error;
use tokio::runtime::Runtime;

pub mod chart;
pub mod configuration;
pub mod errors;
pub mod logger;
pub mod ollama;
pub mod project;
pub mod recurring_task;
pub mod repository;
pub mod task;

use configuration::manager::ConfigurationManager;
use configuration::manager::ConfigurationMode;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Runtime error: {0}")]
    Runtime(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
}

fn detect_mode() -> ConfigurationMode {
    if cfg!(debug_assertions) {
        ConfigurationMode::Development
    } else {
        ConfigurationMode::Desktop
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logger::init(cfg!(debug_assertions));
    tauri::Builder::default()
        .setup(move |app| {
            // Initialize error handler
            let error_handler = errors::ErrorHandler::initialise(detect_mode());
            app.manage(error_handler);

            // Load Configuration
            let configuration_manager = ConfigurationManager::load(detect_mode())
                .map_err(|_| AppError::Configuration("Failed to load configuration".to_string()))?;
            let config_clone = configuration_manager.clone();

            // Create the database
            let rt = Runtime::new().map_err(|e| Box::new(AppError::Runtime(e)))?;
            let db_pool = rt
                .block_on(async move {
                    log::debug!("Setting up db connection pool");
                    let db_pool = SqlitePool::connect(
                        configuration_manager
                            .configuration
                            .db_path
                            .to_str()
                            .ok_or_else(|| {
                                AppError::Configuration("Invalid database path".to_string())
                            })?,
                    )
                    .await
                    .map_err(|e| AppError::Database(e.to_string()))?;

                    sqlx::migrate!("./migrations")
                        .run(&db_pool)
                        .await
                        .map_err(|e| AppError::Database(e.to_string()))?;

                    log::debug!("Migrations run successfully");

                    Ok::<SqlitePool, AppError>(db_pool)
                })
                .map_err(|e| Box::new(e))?;

            // Create and manage the repository provider
            let repository_provider = repository::RepositoryProvider::new(db_pool.clone());
            app.manage(repository_provider);
            app.manage(db_pool);

            app.manage(AsyncMutex::new(config_clone.configuration.clone()));
            app.manage(config_clone);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Project commands
            project::tauri::actions::create_project_command,
            project::tauri::actions::update_project_command,
            project::tauri::actions::archive_project_command,
            project::tauri::queries::load_projects_command,
            project::tauri::queries::load_project_details_command,
            project::tauri::queries::load_favorite_projects_command,
            project::tauri::queries::count_open_tasks_for_project_command,
            project::tauri::queries::add_favorite_project_command,
            project::tauri::queries::remove_favorite_project_command,
            // Task commands
            task::tauri::actions::create_task_command,
            task::tauri::actions::update_task_command,
            task::tauri::actions::delete_task_command,
            task::tauri::actions::complete_task_command,
            task::tauri::actions::create_subtask_for_task_command,
            task::tauri::actions::promote_task_to_project_command,
            task::tauri::queries::load_tasks_command,
            task::tauri::queries::load_task_by_id_command,
            task::tauri::queries::load_tasks_inbox_command,
            task::tauri::queries::load_tasks_due_today_command,
            task::tauri::queries::load_completed_tasks_command,
            task::tauri::queries::load_subtasks_for_task_command,
            task::tauri::queries::load_completed_subtasks_for_task_command,
            task::tauri::queries::load_task_activity_statistics_command,
            task::tauri::queries::load_tasks_by_project_command,
            // Chart commands
            chart::tauri::queries::load_rolling_week_day_charts_command,
            chart::tauri::queries::load_project_activity_stats_command,
            // Configuration commands
            configuration::tauri::queries::load_configuration_command,
            // Ollama commands
            ollama::tauri::get_tasks_prioritization,
            ollama::tauri::get_quick_task,
            // Recurring task commands
            recurring_task::tauri::actions::setup_recurring_task_command,
            recurring_task::tauri::actions::update_recurring_task_command,
            recurring_task::tauri::actions::delete_recurring_task_command,
            recurring_task::tauri::queries::get_recurring_task_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
