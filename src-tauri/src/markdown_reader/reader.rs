use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MarkdownTaskStatus {
    Pending,
    Completed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarkdownTask {
    pub title: String,
    pub status: MarkdownTaskStatus,
    pub file_path: String, // Store path as string for serialization
    pub line_number: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum ReaderError {
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    #[error("Failed to read directory: {0}")]
    ReadDirError(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub fn read_markdown_tasks(folder_path: &Path) -> Result<Vec<MarkdownTask>, ReaderError> {
    if !folder_path.is_dir() {
        return Err(ReaderError::DirectoryNotFound(
            folder_path.to_string_lossy().into_owned(),
        ));
    }

    let mut tasks = Vec::new();

    let entries = fs::read_dir(folder_path).map_err(|e| {
        ReaderError::ReadDirError(format!("Failed to read {}: {}", folder_path.display(), e))
    })?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let file = fs::File::open(&path)?;
            let reader = io::BufReader::new(file);

            for (index, line_result) in reader.lines().enumerate() {
                let line = line_result?;
                let trimmed_line = line.trim();

                if let Some(title) = trimmed_line.strip_prefix("- [ ] ") {
                    tasks.push(MarkdownTask {
                        title: title.trim().to_string(),
                        status: MarkdownTaskStatus::Pending,
                        file_path: path.to_string_lossy().into_owned(),
                        line_number: index + 1, // Line numbers are 1-based
                    });
                } else if let Some(title) = trimmed_line.strip_prefix("[x] ") {
                    tasks.push(MarkdownTask {
                        title: title.trim().to_string(),
                        status: MarkdownTaskStatus::Completed,
                        file_path: path.to_string_lossy().into_owned(),
                        line_number: index + 1, // Line numbers are 1-based
                    });
                }
            }
        }
    }

    Ok(tasks)
}
