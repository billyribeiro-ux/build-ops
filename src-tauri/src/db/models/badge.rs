use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Badge {
    pub id: String,
    pub program_id: String,
    pub badge_type: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub earned_at: String,
    pub metadata: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeProgress {
    pub badge_type: String,
    pub title: String,
    pub description: String,
    pub current_value: i32,
    pub target_value: i32,
    pub progress_percent: f64,
    pub is_earned: bool,
}
