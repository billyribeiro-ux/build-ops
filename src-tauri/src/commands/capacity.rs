use crate::db::models::{UserCapacityProfile, UpdateCapacityInput};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn get_capacity_profile(
    pool: State<'_, SqlitePool>,
) -> Result<UserCapacityProfile> {
    let profile = sqlx::query_as::<_, UserCapacityProfile>(
        "SELECT * FROM user_capacity_profiles WHERE user_id = 'default' LIMIT 1"
    )
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound("Capacity profile not found".to_string()))?;
    
    Ok(profile)
}

#[tauri::command]
pub async fn update_capacity_profile(
    pool: State<'_, SqlitePool>,
    input: UpdateCapacityInput,
) -> Result<UserCapacityProfile> {
    let mut query = String::from("UPDATE user_capacity_profiles SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')");
    
    if input.default_daily_minutes.is_some() {
        query.push_str(", default_daily_minutes = ?");
    }
    if input.weekly_study_days.is_some() {
        query.push_str(", weekly_study_days = ?");
    }
    if input.preferred_start_time.is_some() {
        query.push_str(", preferred_start_time = ?");
    }
    if input.max_deep_days_per_week.is_some() {
        query.push_str(", max_deep_days_per_week = ?");
    }
    if input.break_pattern.is_some() {
        query.push_str(", break_pattern = ?");
    }
    if input.timezone.is_some() {
        query.push_str(", timezone = ?");
    }
    
    query.push_str(" WHERE user_id = 'default' RETURNING *");
    
    let mut q = sqlx::query_as::<_, UserCapacityProfile>(&query);
    
    if let Some(val) = input.default_daily_minutes {
        q = q.bind(val);
    }
    if let Some(val) = input.weekly_study_days {
        q = q.bind(val);
    }
    if let Some(val) = input.preferred_start_time {
        q = q.bind(val);
    }
    if let Some(val) = input.max_deep_days_per_week {
        q = q.bind(val);
    }
    if let Some(val) = input.break_pattern {
        q = q.bind(val);
    }
    if let Some(val) = input.timezone {
        q = q.bind(val);
    }
    
    let profile = q.fetch_optional(pool.inner()).await?
        .ok_or_else(|| crate::error::AppError::NotFound("Capacity profile not found".to_string()))?;
    
    Ok(profile)
}
