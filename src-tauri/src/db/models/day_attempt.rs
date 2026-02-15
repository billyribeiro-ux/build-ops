use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayAttempt {
    pub id: String,
    pub day_plan_id: String,
    pub attempt_number: i32,
    pub status: String,
    pub is_draft: bool,
    pub score_implementation: i32,
    pub score_code_quality: i32,
    pub score_accessibility: i32,
    pub score_performance: i32,
    pub score_quiz: i32,
    pub total_score: i32,
    pub daily_summary: String,
    pub what_went_well: String,
    pub what_to_improve: String,
    pub key_learnings: String,
    pub memory_rebuild_passed: bool,
    pub memory_rebuild_notes: String,
    pub actual_minutes: i32,
    pub started_at: String,
    pub submitted_at: Option<String>,
    pub last_autosave: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDayAttemptInput {
    pub day_plan_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDayAttemptInput {
    pub daily_summary: Option<String>,
    pub what_went_well: Option<String>,
    pub what_to_improve: Option<String>,
    pub key_learnings: Option<String>,
    pub memory_rebuild_passed: Option<bool>,
    pub memory_rebuild_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitScoresInput {
    pub score_implementation: i32,
    pub score_code_quality: i32,
    pub score_accessibility: i32,
    pub score_performance: i32,
    pub score_quiz: i32,
    pub daily_summary: String,
    pub what_went_well: String,
    pub what_to_improve: String,
    pub key_learnings: String,
    pub memory_rebuild_passed: bool,
    pub memory_rebuild_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayAttemptSummary {
    pub id: String,
    pub day_plan_id: String,
    pub day_title: String,
    pub day_number: i32,
    pub attempt_number: i32,
    pub status: String,
    pub total_score: i32,
    pub memory_rebuild_passed: bool,
    pub actual_minutes: i32,
    pub submitted_at: Option<String>,
    pub created_at: String,
}
