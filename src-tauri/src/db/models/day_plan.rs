use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayPlan {
    pub id: String,
    pub program_id: String,
    pub module_id: String,
    pub title: String,
    pub day_number: i32,
    pub version: i32,
    pub status: String,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub files_to_create: String,
    pub success_criteria: String,
    pub stretch_challenge: String,
    pub notes: String,
    pub estimated_minutes: i32,
    pub memory_rebuild_minutes: i32,
    pub min_minutes: i32,
    pub recommended_minutes: i32,
    pub deep_minutes: i32,
    pub complexity_level: i32,
    pub focus_blocks: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusBlock {
    pub session_type: String,
    pub minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDayPlanInput {
    pub program_id: String,
    pub module_id: String,
    pub title: String,
    pub day_number: i32,
    pub syntax_targets: String,
    pub implementation_brief: String,
    pub min_minutes: Option<i32>,
    pub recommended_minutes: Option<i32>,
    pub deep_minutes: Option<i32>,
    pub complexity_level: Option<i32>,
}
