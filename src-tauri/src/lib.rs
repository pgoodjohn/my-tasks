use sqlx::sqlite::SqlitePool;
use std::sync::Mutex;
use tauri::Manager;
use tokio;

mod commands;
mod configuration;
mod project;
mod storage;
mod task;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let configuration = configuration::Configuration::init().unwrap();
            log::info!("Starting My Tasks!");
            log::debug!("Initialising app with configuration: {:?}", configuration);
            // Create a new Tokio runtime
            let rt = tokio::runtime::Runtime::new().unwrap();

            // Use the runtime to block on the async connection
            let db_pool = rt.block_on(async move {
                log::debug!("Setting up db connection pool");
                let db_pool = SqlitePool::connect(
                    configuration::Configuration::db_path(cfg!(debug_assertions))
                        .to_str()
                        .unwrap(),
                )
                .await
                .unwrap();
                sqlx::migrate!("./migrations").run(&db_pool).await.unwrap();
                log::debug!("Migrations run successfully");

                return db_pool;
            });
            app.manage(Mutex::new(db_pool.clone()));

            app.manage(Mutex::new(configuration));

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            configuration::add_project_to_favourites_command,
            configuration::load_configuration_command,
            configuration::remove_project_from_favourites_command,
            project::commands::archive_project_command,
            project::commands::create_project_command,
            project::commands::load_projects_command,
            project::commands::update_project_command,
            project::commands::count_open_tasks_for_project_command,
            project::commands::load_project_details_command,
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
