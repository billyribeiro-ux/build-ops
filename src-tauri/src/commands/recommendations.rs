use crate::db::models::TimeRecommendation;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn generate_recommendations(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<TimeRecommendation>, String> {
    let session_data: Vec<(String, i32, i32)> = sqlx::query_as(
        "SELECT session_type, planned_minutes, actual_minutes 
         FROM day_sessions 
         WHERE status = 'done' 
         AND created_at > datetime('now', '-14 days')
         ORDER BY created_at DESC"
    )
    .fetch_all(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    let mut recommendations = Vec::new();
    
    let build_sessions: Vec<&(String, i32, i32)> = session_data
        .iter()
        .filter(|(t, _, _)| t == "build")
        .collect();
    
    if build_sessions.len() >= 5 {
        let avg_overrun: f64 = build_sessions
            .iter()
            .map(|(_, planned, actual)| (*actual - *planned) as f64)
            .sum::<f64>() / build_sessions.len() as f64;
        
        if avg_overrun > 20.0 {
            let rec = create_recommendation(
                pool.clone(),
                "increase_build",
                "Increase Build Session Time",
                &format!("You consistently overrun build blocks by +{:.0} minutes. Consider increasing build allocation from 90 → {} minutes.", avg_overrun, 90 + (avg_overrun * 0.7) as i32),
                &format!("Analysis of {} build sessions over 14 days", build_sessions.len()),
                0.85,
            ).await.map_err(|e| e.to_string())?;
            recommendations.push(rec);
        }
    }
    
    let daily_totals: Vec<i32> = sqlx::query_scalar(
        "SELECT SUM(actual_minutes) as total
         FROM day_sessions
         WHERE status = 'done'
         AND DATE(created_at) > DATE('now', '-7 days')
         GROUP BY DATE(created_at)
         HAVING total > 240"
    )
    .fetch_all(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    if daily_totals.len() > 3 {
        let rec = create_recommendation(
            pool.clone(),
            "decrease_deep",
            "Reduce Deep Work Days",
            &format!("You've had {} days over 4 hours in the past week. Consider capping deep sessions to 3.5 hours to avoid burnout.", daily_totals.len()),
            "Weekly deep work analysis",
            0.75,
        ).await.map_err(|e| e.to_string())?;
        recommendations.push(rec);
    }
    
    Ok(recommendations)
}

#[tauri::command]
pub async fn list_recommendations(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<TimeRecommendation>, String> {
    let recommendations = sqlx::query_as::<_, TimeRecommendation>(
        "SELECT * FROM time_recommendations 
         WHERE is_applied = 0 AND is_dismissed = 0
         ORDER BY confidence_score DESC, created_at DESC"
    )
    .fetch_all(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    Ok(recommendations)
}

#[tauri::command]
pub async fn apply_recommendation(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<TimeRecommendation, String> {
    let rec = sqlx::query_as::<_, TimeRecommendation>(
        "UPDATE time_recommendations 
         SET is_applied = 1, applied_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ? 
         RETURNING *"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Recommendation {} not found", id))?;
    
    Ok(rec)
}

#[tauri::command]
pub async fn dismiss_recommendation(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<TimeRecommendation, String> {
    let rec = sqlx::query_as::<_, TimeRecommendation>(
        "UPDATE time_recommendations 
         SET is_dismissed = 1, dismissed_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
         WHERE id = ? 
         RETURNING *"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await.map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Recommendation {} not found", id))?;
    
    Ok(rec)
}

async fn create_recommendation(
    pool: State<'_, SqlitePool>,
    rec_type: &str,
    title: &str,
    description: &str,
    data_source: &str,
    confidence: f64,
) -> Result<TimeRecommendation, String> {
    let id = Uuid::new_v4().to_string();
    
    let rec = sqlx::query_as::<_, TimeRecommendation>(
        "INSERT INTO time_recommendations (
            id, user_id, recommendation_type, title, description, data_source, confidence_score
         ) VALUES (?, 'default', ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(rec_type)
    .bind(title)
    .bind(description)
    .bind(data_source)
    .bind(confidence)
    .fetch_one(pool.inner())
    .await.map_err(|e| e.to_string())?;
    
    Ok(rec)
}
