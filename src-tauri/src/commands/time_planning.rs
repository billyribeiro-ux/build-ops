use crate::db::models::{DayPlan, UserCapacityProfile, DaySession, GeneratedPlan, PlannedSession, FocusBlock, CreateSessionInput};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;
use chrono::{DateTime, Utc, Duration, NaiveTime};

#[tauri::command]
pub async fn plan_my_day(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
    day_attempt_id: String,
) -> Result<GeneratedPlan> {
    let day_plan: DayPlan = sqlx::query_as(
        "SELECT * FROM day_plans WHERE id = ?"
    )
    .bind(&day_plan_id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound("Day plan not found".to_string()))?;
    
    let capacity: UserCapacityProfile = sqlx::query_as(
        "SELECT * FROM user_capacity_profiles WHERE user_id = 'default' LIMIT 1"
    )
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound("Capacity profile not found".to_string()))?;
    
    let historical_data: Vec<(String, i32, i32)> = sqlx::query_as(
        "SELECT ds.session_type, ds.planned_minutes, ds.actual_minutes 
         FROM day_sessions ds
         JOIN day_attempts da ON ds.day_attempt_id = da.id
         JOIN day_plans dp ON da.day_plan_id = dp.id
         WHERE dp.complexity_level = ?
         AND ds.status = 'done'
         LIMIT 50"
    )
    .bind(day_plan.complexity_level)
    .fetch_all(pool.inner())
    .await?;
    
    let focus_blocks: Vec<FocusBlock> = serde_json::from_str(&day_plan.focus_blocks)
        .unwrap_or_else(|_| vec![]);
    
    let adjusted_blocks = adjust_blocks_with_history(focus_blocks, &historical_data);
    
    let start_time = parse_time(&capacity.preferred_start_time);
    let break_pattern = parse_break_pattern(&capacity.break_pattern);
    
    let mut planned_sessions = Vec::new();
    let mut current_time = Utc::now()
        .date_naive()
        .and_time(start_time)
        .and_utc();
    
    for block in &adjusted_blocks {
        let session_start = current_time;
        let session_end = current_time + Duration::minutes(block.minutes as i64);
        
        planned_sessions.push(PlannedSession {
            session_type: block.session_type.clone(),
            planned_minutes: block.minutes,
            start_time: session_start.to_rfc3339(),
            end_time: session_end.to_rfc3339(),
            description: get_session_description(&block.session_type),
        });
        
        current_time = session_end + Duration::minutes(break_pattern.0 as i64);
    }
    
    let total_minutes: i32 = adjusted_blocks.iter().map(|b| b.minutes).sum();
    let estimated_end_time = current_time.to_rfc3339();
    
    for session in &planned_sessions {
        let input = CreateSessionInput {
            day_attempt_id: day_attempt_id.clone(),
            session_type: session.session_type.clone(),
            planned_minutes: session.planned_minutes,
        };
        
        crate::commands::sessions::create_session(pool.clone(), input).await?;
    }
    
    Ok(GeneratedPlan {
        sessions: planned_sessions,
        total_minutes,
        estimated_end_time,
    })
}

fn adjust_blocks_with_history(
    blocks: Vec<FocusBlock>,
    history: &[(String, i32, i32)],
) -> Vec<FocusBlock> {
    let mut adjusted = blocks.clone();
    
    for block in &mut adjusted {
        let relevant_history: Vec<&(String, i32, i32)> = history
            .iter()
            .filter(|(session_type, _, _)| session_type == &block.session_type)
            .collect();
        
        if !relevant_history.is_empty() {
            let avg_overrun: f64 = relevant_history
                .iter()
                .map(|(_, planned, actual)| (*actual - *planned) as f64)
                .sum::<f64>() / relevant_history.len() as f64;
            
            if avg_overrun > 10.0 {
                block.minutes += (avg_overrun * 0.5) as i32;
            }
        }
    }
    
    adjusted
}

fn parse_time(time_str: &str) -> NaiveTime {
    NaiveTime::parse_from_str(time_str, "%H:%M")
        .unwrap_or_else(|_| NaiveTime::from_hms_opt(18, 0, 0).unwrap())
}

fn parse_break_pattern(pattern: &str) -> (i32, i32) {
    let parts: Vec<&str> = pattern.split('/').collect();
    if parts.len() == 2 {
        let work = parts[0].parse().unwrap_or(50);
        let break_time = parts[1].parse().unwrap_or(10);
        (work, break_time)
    } else {
        (50, 10)
    }
}

fn get_session_description(session_type: &str) -> String {
    match session_type {
        "learn" => "Review concepts and documentation".to_string(),
        "build" => "Implement core features".to_string(),
        "debug" => "Debug and fix issues".to_string(),
        "rebuild" => "Memory rebuild from scratch".to_string(),
        "quiz" => "Quiz and reflection".to_string(),
        "review" => "Review and refactor".to_string(),
        _ => "Work session".to_string(),
    }
}
