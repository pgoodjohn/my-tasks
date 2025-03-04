use crate::errors::handle_error;
use crate::repository::RepositoryProvider;
use crate::task::repository::TaskRepository;
use crate::task::Task;

#[tauri::command]
pub async fn get_tasks_prioritization(
    repository_provider: tauri::State<'_, RepositoryProvider>,
) -> Result<String, String> {
    let mut task_repository = repository_provider
        .task_repository()
        .await
        .map_err(|e| handle_error(&e))?;

    let tasks = task_repository
        .find_all_filtered_by_completed(false)
        .await
        .map_err(|e| handle_error(&e))?;

    let tasks_text = format_tasks_for_ollama(&tasks);

    let analysis = super::get_task_prioritization(tasks_text)
        .await
        .map_err(|e| handle_error(&*e))?;

    log::debug!("Analysis: {:?}", analysis);

    // Return JSON response
    Ok(serde_json::to_string(&analysis).unwrap())
}

fn format_tasks_for_ollama(tasks: &[Task]) -> String {
    tasks
        .iter()
        .map(|task| {
            format!(
                "- {} (Due: {})",
                task.title,
                task.due_at_utc
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "No due date".to_string())
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
