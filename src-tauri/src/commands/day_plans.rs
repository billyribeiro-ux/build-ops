use crate::db::models::{DayPlan, CreateDayPlanInput, FocusBlock};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_day_plan(
    pool: State<'_, SqlitePool>,
    input: CreateDayPlanInput,
) -> Result<DayPlan> {
    let id = Uuid::new_v4().to_string();
    
    let min_minutes = input.min_minutes.unwrap_or(90);
    let recommended_minutes = input.recommended_minutes.unwrap_or(120);
    let deep_minutes = input.deep_minutes.unwrap_or(180);
    let complexity_level = input.complexity_level.unwrap_or(3);
    
    let focus_blocks = generate_default_focus_blocks(complexity_level);
    let focus_blocks_json = serde_json::to_string(&focus_blocks)?;
    
    let day_plan = sqlx::query_as::<_, DayPlan>(
        "INSERT INTO day_plans (
            id, program_id, module_id, title, day_number, 
            syntax_targets, implementation_brief,
            min_minutes, recommended_minutes, deep_minutes, complexity_level, focus_blocks
         ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.program_id)
    .bind(&input.module_id)
    .bind(&input.title)
    .bind(input.day_number)
    .bind(&input.syntax_targets)
    .bind(&input.implementation_brief)
    .bind(min_minutes)
    .bind(recommended_minutes)
    .bind(deep_minutes)
    .bind(complexity_level)
    .bind(&focus_blocks_json)
    .fetch_one(pool.inner())
    .await?;
    
    Ok(day_plan)
}

#[tauri::command]
pub async fn get_day_plan(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DayPlan> {
    let day_plan = sqlx::query_as::<_, DayPlan>(
        "SELECT * FROM day_plans WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Day plan {} not found", id)))?;
    
    Ok(day_plan)
}

#[tauri::command]
pub async fn list_day_plans(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<DayPlan>> {
    let day_plans = sqlx::query_as::<_, DayPlan>(
        "SELECT * FROM day_plans WHERE program_id = ? ORDER BY day_number ASC"
    )
    .bind(&program_id)
    .fetch_all(pool.inner())
    .await?;
    
    Ok(day_plans)
}

fn generate_default_focus_blocks(complexity: i32) -> Vec<FocusBlock> {
    match complexity {
        1 | 2 => vec![
            FocusBlock { session_type: "learn".to_string(), minutes: 20 },
            FocusBlock { session_type: "build".to_string(), minutes: 60 },
            FocusBlock { session_type: "quiz".to_string(), minutes: 10 },
        ],
        3 => vec![
            FocusBlock { session_type: "learn".to_string(), minutes: 30 },
            FocusBlock { session_type: "build".to_string(), minutes: 90 },
            FocusBlock { session_type: "rebuild".to_string(), minutes: 20 },
            FocusBlock { session_type: "quiz".to_string(), minutes: 10 },
        ],
        4 | 5 => vec![
            FocusBlock { session_type: "learn".to_string(), minutes: 30 },
            FocusBlock { session_type: "build".to_string(), minutes: 120 },
            FocusBlock { session_type: "debug".to_string(), minutes: 30 },
            FocusBlock { session_type: "rebuild".to_string(), minutes: 30 },
            FocusBlock { session_type: "quiz".to_string(), minutes: 20 },
        ],
        _ => vec![
            FocusBlock { session_type: "learn".to_string(), minutes: 30 },
            FocusBlock { session_type: "build".to_string(), minutes: 90 },
            FocusBlock { session_type: "quiz".to_string(), minutes: 10 },
        ],
    }
}
