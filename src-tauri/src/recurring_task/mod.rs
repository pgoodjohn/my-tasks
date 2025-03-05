use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::fmt::Hyphenated;
use uuid::Uuid;

pub mod manager;
pub mod repository;
pub mod tauri;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl ToString for Frequency {
    fn to_string(&self) -> String {
        match self {
            Frequency::Daily => "daily".to_string(),
            Frequency::Weekly => "weekly".to_string(),
            Frequency::Monthly => "monthly".to_string(),
            Frequency::Yearly => "yearly".to_string(),
        }
    }
}

impl TryFrom<String> for Frequency {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "daily" => Ok(Frequency::Daily),
            "weekly" => Ok(Frequency::Weekly),
            "monthly" => Ok(Frequency::Monthly),
            "yearly" => Ok(Frequency::Yearly),
            _ => Err(format!("Invalid frequency: {}", value)),
        }
    }
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
        Self {
            id: Uuid::now_v7(),
            task_id,
            frequency: frequency.to_string(),
            interval,
            next_due_at_utc,
            created_at_utc: Utc::now(),
            updated_at_utc: Utc::now(),
        }
    }

    pub fn frequency(&self) -> Result<Frequency, String> {
        self.frequency.clone().try_into()
    }
}
