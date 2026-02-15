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
pub struct UpdateAttemptInput {
    pub status: Option<String>,
    pub score_implementation: Option<i32>,
    pub score_code_quality: Option<i32>,
    pub score_accessibility: Option<i32>,
    pub score_performance: Option<i32>,
    pub score_quiz: Option<i32>,
    pub memory_rebuild_completed: Option<i32>,
    pub memory_rebuild_passed: Option<i32>,
    pub memory_rebuild_notes: Option<String>,
    pub what_broke: Option<String>,
    pub why_broke: Option<String>,
    pub how_fixed: Option<String>,
    pub refactor_tomorrow: Option<String>,
    pub daily_summary: Option<String>,
    pub exercise_notes: Option<String>,
    pub code_snapshot: Option<String>,
    pub actual_minutes: Option<i32>,
}
