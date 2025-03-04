use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::{fmt::Hyphenated, Uuid};

use crate::task::Task;

pub mod manager;
pub mod repository;
pub mod tauri;
mod test;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Project {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    pub title: String,
    pub emoji: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,
    pub archived_at_utc: Option<DateTime<Utc>>,
    #[serde(rename(serialize = "isFavorite"))]
    pub is_favorite: bool,
}

#[derive(Debug, Serialize)]
pub struct ProjectDetail {
    pub project: Project,
    pub tasks: Vec<Task>,
}
