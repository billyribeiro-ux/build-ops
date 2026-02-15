use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Streak {
    pub id: String,
    pub program_id: String,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub last_activity_date: String,
    pub freezes_available: i32,
    pub freezes_used_this_month: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StreakFreeze {
    pub id: String,
    pub program_id: String,
    pub used_date: String,
    pub reason: String,
    pub created_at: String,
}
