use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Program {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_days: i32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProgramSummary {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_days: i32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub days_count: i32,
    pub completed_days: i32,
    pub latest_score: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramStats {
    pub total_days: i32,
    pub completed_days: i32,
    pub blocked_days: i32,
    pub average_score: f64,
    pub current_streak: i32,
    pub total_time_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProgramInput {
    pub title: String,
    pub description: String,
    pub target_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProgramInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}
