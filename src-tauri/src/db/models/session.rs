use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DaySession {
    pub id: String,
    pub day_attempt_id: String,
    pub session_type: String,
    pub planned_minutes: i32,
    pub actual_minutes: i32,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub status: String,
    pub notes: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionInput {
    pub day_attempt_id: String,
    pub session_type: String,
    pub planned_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInterruption {
    pub id: String,
    pub session_id: String,
    pub interruption_type: String,
    pub duration_seconds: i32,
    pub notes: String,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPlan {
    pub sessions: Vec<PlannedSession>,
    pub total_minutes: i32,
    pub estimated_end_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedSession {
    pub session_type: String,
    pub planned_minutes: i32,
    pub start_time: String,
    pub end_time: String,
    pub description: String,
}
