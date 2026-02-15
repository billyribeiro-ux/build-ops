use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExerciseEntry {
    pub id: String,
    pub day_attempt_id: String,
    pub language: String,
    pub code: String,
    pub notes: String,
    pub order_index: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExerciseEntryInput {
    pub day_attempt_id: String,
    pub language: String,
    pub code: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExerciseEntryInput {
    pub language: Option<String>,
    pub code: Option<String>,
    pub notes: Option<String>,
}
