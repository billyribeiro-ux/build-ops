use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DayDependency {
    pub id: String,
    pub day_plan_id: String,
    pub depends_on_day_plan_id: String,
    pub dependency_type: String,
    pub minimum_score: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDependencyInput {
    pub day_plan_id: String,
    pub depends_on_day_plan_id: String,
    pub dependency_type: String,
    pub minimum_score: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyStatus {
    pub dependency_id: String,
    pub depends_on_day_plan_id: String,
    pub depends_on_title: String,
    pub depends_on_day_number: i32,
    pub dependency_type: String,
    pub minimum_score: i32,
    pub is_met: bool,
    pub best_score: Option<i32>,
    pub best_attempt_status: Option<String>,
}
