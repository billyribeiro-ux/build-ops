use crate::db::models::{TimeAnalytics, AccuracyPoint, FocusMetricsDaily};
use sqlx::SqlitePool;
use tauri::State;
use chrono::Utc;

#[tauri::command]
pub async fn get_time_analytics(
    pool: State<'_, SqlitePool>,
) -> Result<TimeAnalytics, String> {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    
    let today_metrics: Option<(i32, i32)> = sqlx::query_as(
        "SELECT total_planned_minutes, total_actual_minutes 
         FROM focus_metrics_daily 
         WHERE date = ? AND user_id = 'default'"
    )
    .bind(&today)
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let (today_planned, today_actual) = today_metrics.unwrap_or((0, 0));
    
    let week_total: Option<i32> = sqlx::query_scalar(
        "SELECT SUM(total_actual_minutes) 
         FROM focus_metrics_daily 
         WHERE date >= DATE('now', '-7 days') AND user_id = 'default'"
    )
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let capacity: (i32, i32) = sqlx::query_as(
        "SELECT default_daily_minutes, weekly_study_days 
         FROM user_capacity_profiles 
         WHERE user_id = 'default' LIMIT 1"
    )
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?
    .unwrap_or((180, 5));
    
    let week_target = capacity.0 * capacity.1;
    
    let accuracy_data: Vec<(String, i32, i32)> = sqlx::query_as(
        "SELECT date, total_planned_minutes, total_actual_minutes 
         FROM focus_metrics_daily 
         WHERE date >= DATE('now', '-14 days') AND user_id = 'default'
         ORDER BY date ASC"
    )
    .fetch_all(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let accuracy_trend: Vec<AccuracyPoint> = accuracy_data
        .into_iter()
        .map(|(date, planned, actual)| {
            let variance = if planned > 0 {
                ((actual - planned) as f64 / planned as f64) * 100.0
            } else {
                0.0
            };
            AccuracyPoint {
                date,
                planned,
                actual,
                variance,
            }
        })
        .collect();
    
    let avg_efficiency: Option<f64> = sqlx::query_scalar(
        "SELECT AVG(focus_efficiency) 
         FROM focus_metrics_daily 
         WHERE date >= DATE('now', '-7 days') AND user_id = 'default'"
    )
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let deep_days_used: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) 
         FROM focus_metrics_daily 
         WHERE date >= DATE('now', 'weekday 0', '-7 days')
         AND deep_work_minutes >= 180 
         AND user_id = 'default'"
    )
    .fetch_one(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    Ok(TimeAnalytics {
        today_planned,
        today_actual,
        week_total: week_total.unwrap_or(0),
        week_target,
        accuracy_trend,
        focus_efficiency: avg_efficiency.unwrap_or(0.0),
        deep_days_used,
        deep_days_limit: capacity.1,
    })
}

#[tauri::command]
pub async fn update_daily_metrics(
    pool: State<'_, SqlitePool>,
    date: String,
) -> Result<FocusMetricsDaily, String> {
    let sessions: Vec<(i32, i32)> = sqlx::query_as(
        "SELECT planned_minutes, actual_minutes 
         FROM day_sessions 
         WHERE DATE(created_at) = ? AND status = 'done'"
    )
    .bind(&date)
    .fetch_all(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let total_planned: i32 = sessions.iter().map(|(p, _)| p).sum();
    let total_actual: i32 = sessions.iter().map(|(_, a)| a).sum();
    
    let variance = if total_planned > 0 {
        ((total_actual - total_planned) as f64 / total_planned as f64) * 100.0
    } else {
        0.0
    };
    
    let completion_rate = if total_planned > 0 {
        (total_actual as f64 / total_planned as f64) * 100.0
    } else {
        0.0
    };
    
    let deep_work_minutes: i32 = sqlx::query_scalar(
        "SELECT SUM(actual_minutes) 
         FROM day_sessions 
         WHERE DATE(created_at) = ? 
         AND session_type IN ('build', 'debug', 'rebuild') 
         AND status = 'done'"
    )
    .bind(&date)
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?
    .unwrap_or(0);
    
    let interruption_count: i32 = sqlx::query_scalar(
        "SELECT COUNT(*) 
         FROM session_interruptions si
         JOIN day_sessions ds ON si.session_id = ds.id
         WHERE DATE(ds.created_at) = ?"
    )
    .bind(&date)
    .fetch_one(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let focus_efficiency = if total_actual > 0 {
        (deep_work_minutes as f64 / total_actual as f64) * 100.0
    } else {
        0.0
    };
    
    let id = uuid::Uuid::new_v4().to_string();
    
    let metrics = sqlx::query_as::<_, FocusMetricsDaily>(
        "INSERT INTO focus_metrics_daily (
            id, date, user_id, total_planned_minutes, total_actual_minutes,
            variance_percentage, completion_rate, focus_efficiency,
            deep_work_minutes, interruption_count
         ) VALUES (?, ?, 'default', ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(date, user_id) DO UPDATE SET
            total_planned_minutes = excluded.total_planned_minutes,
            total_actual_minutes = excluded.total_actual_minutes,
            variance_percentage = excluded.variance_percentage,
            completion_rate = excluded.completion_rate,
            focus_efficiency = excluded.focus_efficiency,
            deep_work_minutes = excluded.deep_work_minutes,
            interruption_count = excluded.interruption_count,
            updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         RETURNING *"
    )
    .bind(&id)
    .bind(&date)
    .bind(total_planned)
    .bind(total_actual)
    .bind(variance)
    .bind(completion_rate)
    .bind(focus_efficiency)
    .bind(deep_work_minutes)
    .bind(interruption_count)
    .fetch_one(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    Ok(metrics)
}
