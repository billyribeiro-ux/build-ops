use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SpacedRepetition {
    pub id: String,
    pub day_plan_id: String,
    pub concept_tag_id: String,
    pub easiness_factor: f64,
    pub interval_days: i32,
    pub repetition_count: i32,
    pub last_review_date: String,
    pub next_review_date: String,
    pub last_score: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordReviewInput {
    pub day_plan_id: String,
    pub concept_tag_id: String,
    pub score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DueReview {
    pub id: String,
    pub day_plan_id: String,
    pub day_title: String,
    pub day_number: i32,
    pub concept_tag_id: String,
    pub concept_name: String,
    pub next_review_date: String,
    pub interval_days: i32,
}
