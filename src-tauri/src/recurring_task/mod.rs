use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use strum_macros;
use uuid::fmt::Hyphenated;
use uuid::Uuid;

pub mod manager;
pub mod repository;
pub mod tauri;

#[derive(Debug, Serialize, Deserialize, Clone, strum_macros::Display, strum_macros::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecurringTask {
    #[sqlx(try_from = "Hyphenated")]
    pub id: Uuid,
    #[sqlx(try_from = "Hyphenated")]
    pub task_id: Uuid,
    pub frequency: String, // Will be converted to/from Frequency enum
    pub interval: i32,
    pub next_due_at_utc: DateTime<Utc>,
    pub created_at_utc: DateTime<Utc>,
    pub updated_at_utc: DateTime<Utc>,
}

impl RecurringTask {
    pub fn new(
        task_id: Uuid,
        frequency: Frequency,
        interval: i32,
        next_due_at_utc: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            task_id,
            frequency: frequency.to_string(),
            interval,
            next_due_at_utc,
            created_at_utc: now,
            updated_at_utc: now,
        }
    }

    pub fn frequency(&self) -> Result<Frequency, strum::ParseError> {
        self.frequency.parse()
    }
}
