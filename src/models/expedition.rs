use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::DateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Expedition {
    pub id: i64,
    pub url: String,
    pub name: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
}
