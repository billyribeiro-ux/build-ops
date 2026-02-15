use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserCapacityProfile {
    pub id: String,
    pub user_id: String,
    pub default_daily_minutes: i32,
    pub weekly_study_days: i32,
    pub preferred_start_time: String,
    pub max_deep_days_per_week: i32,
    pub break_pattern: String,
    pub timezone: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCapacityInput {
    pub default_daily_minutes: Option<i32>,
    pub weekly_study_days: Option<i32>,
    pub preferred_start_time: Option<String>,
    pub max_deep_days_per_week: Option<i32>,
    pub break_pattern: Option<String>,
    pub timezone: Option<String>,
}
