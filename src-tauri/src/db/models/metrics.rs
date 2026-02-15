use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FocusMetricsDaily {
    pub id: String,
    pub date: String,
    pub user_id: String,
    pub total_planned_minutes: i32,
    pub total_actual_minutes: i32,
    pub variance_percentage: f64,
    pub completion_rate: f64,
    pub focus_efficiency: f64,
    pub deep_work_minutes: i32,
    pub interruption_count: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeAnalytics {
    pub today_planned: i32,
    pub today_actual: i32,
    pub week_total: i32,
    pub week_target: i32,
    pub accuracy_trend: Vec<AccuracyPoint>,
    pub focus_efficiency: f64,
    pub deep_days_used: i32,
    pub deep_days_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyPoint {
    pub date: String,
    pub planned: i32,
    pub actual: i32,
    pub variance: f64,
}
