use crate::configuration::manager::ConfigurationManager;
use crate::errors::handle_error;
use crate::project::repository::ProjectRepository;
use crate::project::Project;
use crate::repository::RepositoryProvider;
use crate::task::repository::TaskRepository;
use crate::task::Task;
use std::collections::HashMap;
use uuid::Uuid;

#[tauri::command]
pub async fn get_tasks_prioritization(
    repository_provider: tauri::State<'_, RepositoryProvider>,
    configuration_manager: tauri::State<'_, ConfigurationManager>,
) -> Result<String, String> {
    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let tasks = task_repository
        .find_all_filtered_by_completed(false)
        .await
        .map_err(|e| handle_error(&e))?;

    // Fetch all projects
    let mut project_repository = repository_provider
        .project_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let projects: Vec<Project> = project_repository
        .find_all()
        .await
        .map_err(|e| handle_error(&e))?;

    // Create a map of project IDs to project details
    let project_details: HashMap<Uuid, &Project> = projects.iter().map(|p| (p.id, p)).collect();

    // Create a map of parent task IDs to titles
    let mut parent_task_titles = HashMap::new();
    for task in &tasks {
        if let Some(parent_id) = task.parent_task_id {
            if !parent_task_titles.contains_key(&parent_id) {
                if let Some(parent_task) = task_repository
                    .find_by_id(parent_id)
                    .await
                    .map_err(|e| handle_error(&e))?
                {
                    parent_task_titles.insert(parent_id, parent_task.title);
                }
            }
        }
    }

    let tasks_text = format_tasks_for_ollama(&tasks, &project_details, &parent_task_titles);

    let config = &configuration_manager.inner().configuration;

    let analysis = super::get_task_prioritization(tasks_text, &config.ollama)
        .await
        .map_err(|e| handle_error(&*e))?;

    log::debug!("Analysis: {:?}", analysis);

    // Return JSON response
    Ok(serde_json::to_string(&analysis).unwrap())
}

fn format_tasks_for_ollama(
    tasks: &[Task],
    project_details: &HashMap<Uuid, &Project>,
    parent_task_titles: &HashMap<Uuid, String>,
) -> String {
    // Group tasks by project_id
    let mut project_tasks: HashMap<Option<Uuid>, Vec<&Task>> = HashMap::new();

    for task in tasks {
        project_tasks.entry(task.project_id).or_default().push(task);
    }

    let mut formatted = String::new();

    // First handle tasks without a project (Inbox)
    if let Some(inbox_tasks) = project_tasks.get(&None) {
        formatted.push_str("# 📥 Inbox Tasks\n\n");
        for task in inbox_tasks {
            formatted.push_str(&format_single_task(task, None, parent_task_titles));
        }
        formatted.push('\n');
    }

    // Then handle tasks with projects
    for (project_id, project_tasks) in project_tasks.iter() {
        if let Some(project_id) = project_id {
            if let Some(project) = project_details.get(project_id) {
                let project_header = format!(
                    "# {} {}\n\n",
                    project.emoji.as_deref().unwrap_or("📁"),
                    project.title
                );
                formatted.push_str(&project_header);

                for task in project_tasks {
                    formatted.push_str(&format_single_task(
                        task,
                        Some(project),
                        parent_task_titles,
                    ));
                }
                formatted.push('\n');
            }
        }
    }

    formatted
}

fn format_single_task(
    task: &Task,
    project: Option<&&Project>,
    parent_task_titles: &HashMap<Uuid, String>,
) -> String {
    let mut task_text = format!("## {}\n", task.title);

    if let Some(desc) = &task.description {
        task_text.push_str(&format!("Description: {}\n", desc));
    }

    if let Some(due_date) = task.due_at_utc {
        task_text.push_str(&format!("Due: {}\n", due_date.format("%Y-%m-%d")));
    }

    if let Some(parent_id) = task.parent_task_id {
        if let Some(parent_title) = parent_task_titles.get(&parent_id) {
            task_text.push_str(&format!("Subtask of: {}\n", parent_title));
        }
    }

    if let Some(project) = project {
        if let Some(color) = &project.color {
            task_text.push_str(&format!("Project Color: {}\n", color));
        }
    }

    task_text.push_str(&format!(
        "Created: {}\n",
        task.created_at_utc.format("%Y-%m-%d")
    ));
    task_text.push_str(&format!(
        "Last Updated: {}\n",
        task.updated_at_utc.format("%Y-%m-%d")
    ));
    task_text.push('\n');

    task_text
}
