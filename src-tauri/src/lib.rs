extern crate r2d2;
extern crate r2d2_sqlite;

use std::sync::Mutex;

mod configuration;
mod project;
mod storage;
mod task;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let configuration = configuration::Configuration::init().unwrap();
    log::info!("Starting My Tasks!");

    let db_pool = storage::setup_database(&configuration).expect("Could not set up database.");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(Mutex::new(configuration))
        .manage(db_pool)
        .invoke_handler(tauri::generate_handler![
            configuration::add_project_to_favourites_command,
            configuration::load_configuration_command,
            configuration::remove_project_from_favourites_command,
            project::archive_project_command,
            project::create_project_command,
            project::load_projects_command,
            project::update_project_command,
            project::detail::load_project_details_command,
            task::complete_task_command,
            task::delete_task_command,
            task::load_task_activity_statistics_command,
            task::load_tasks_command,
            task::load_tasks_due_today_command,
            task::save_task_command,
            task::commands::update_task_command,
            task::count_open_tasks_for_project_command,
            task::load_tasks_with_deadline_command,
            task::load_tasks_inbox_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
