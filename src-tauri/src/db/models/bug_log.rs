use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BugLog {
    pub id: String,
    pub day_attempt_id: String,
    pub category: String,
    pub severity: String,
    pub symptom: String,
    pub root_cause: String,
    pub fix_applied: String,
    pub prevention_strategy: String,
    pub time_to_fix_minutes: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBugLogInput {
    pub day_attempt_id: String,
    pub category: String,
    pub severity: String,
    pub symptom: String,
    pub root_cause: String,
    pub fix_applied: String,
    pub prevention_strategy: String,
    pub time_to_fix_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBugLogInput {
    pub category: Option<String>,
    pub severity: Option<String>,
    pub symptom: Option<String>,
    pub root_cause: Option<String>,
    pub fix_applied: Option<String>,
    pub prevention_strategy: Option<String>,
    pub time_to_fix_minutes: Option<i32>,
}
