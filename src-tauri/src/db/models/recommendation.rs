use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TimeRecommendation {
    pub id: String,
    pub user_id: String,
    pub recommendation_type: String,
    pub title: String,
    pub description: String,
    pub data_source: String,
    pub confidence_score: f64,
    pub is_applied: i32,
    pub is_dismissed: i32,
    pub created_at: String,
    pub applied_at: Option<String>,
    pub dismissed_at: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRecommendationInput {
    pub recommendation_type: String,
    pub title: String,
    pub description: String,
    pub data_source: String,
    pub confidence_score: f64,
}
