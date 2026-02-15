use crate::db::models::{Program, CreateProgramInput, UpdateProgramInput};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_program(
    pool: State<'_, SqlitePool>,
    input: CreateProgramInput,
) -> Result<Program, String> {
    let id = Uuid::new_v4().to_string();
    
    let program = sqlx::query_as::<_, Program>(
        "INSERT INTO programs (id, title, description, target_days) 
         VALUES (?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(input.target_days)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(program)
}

#[tauri::command]
pub async fn get_program(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<Program, String> {
    let program = sqlx::query_as::<_, Program>(
        "SELECT * FROM programs WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Program {} not found", id))?;
    
    Ok(program)
}

#[tauri::command]
pub async fn list_programs(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<crate::db::models::ProgramSummary>, String> {
    let programs = sqlx::query_as::<_, crate::db::models::ProgramSummary>(
        "SELECT 
            p.*,
            COALESCE(COUNT(DISTINCT dp.id), 0) as days_count,
            COALESCE(COUNT(DISTINCT CASE WHEN da.status IN ('passed', 'mastery') THEN da.day_plan_id END), 0) as completed_days,
            (SELECT total_score FROM day_attempts WHERE day_plan_id IN (SELECT id FROM day_plans WHERE program_id = p.id) ORDER BY created_at DESC LIMIT 1) as latest_score
         FROM programs p
         LEFT JOIN day_plans dp ON dp.program_id = p.id
         LEFT JOIN day_attempts da ON da.day_plan_id = dp.id
         WHERE p.status != 'archived'
         GROUP BY p.id
         ORDER BY p.created_at DESC"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(programs)
}

#[tauri::command]
pub async fn update_program(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateProgramInput,
) -> Result<Program, String> {
    let mut query = String::from("UPDATE programs SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')");
    let mut params: Vec<String> = Vec::new();
    
    if let Some(title) = input.title {
        query.push_str(", title = ?");
        params.push(title);
    }
    if let Some(description) = input.description {
        query.push_str(", description = ?");
        params.push(description);
    }
    if let Some(status) = input.status {
        query.push_str(", status = ?");
        params.push(status);
    }
    
    query.push_str(" WHERE id = ? RETURNING *");
    params.push(id.clone());
    
    let mut q = sqlx::query_as::<_, Program>(&query);
    for param in params {
        q = q.bind(param);
    }
    
    let program = q.fetch_optional(pool.inner()).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Program {} not found", id))?;
    
    Ok(program)
}

#[tauri::command]
pub async fn delete_program(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM programs WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn duplicate_program(
    pool: State<'_, SqlitePool>,
    id: String,
    new_title: String,
) -> Result<Program, String> {
    // Get the original program
    let original = get_program(pool.clone(), id.clone()).await?;
    
    // Create new program with same data but new title
    let new_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    let new_program = sqlx::query_as::<_, Program>(
        "INSERT INTO programs (id, title, description, target_days, status, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&new_id)
    .bind(&new_title)
    .bind(&original.description)
    .bind(original.target_days)
    .bind("active")
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    // Copy all modules
    let modules = sqlx::query_as::<_, crate::db::models::Module>(
        "SELECT * FROM modules WHERE program_id = ? ORDER BY order_index"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    for module in modules {
        let module_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO modules (id, program_id, title, description, order_index, color, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&module_id)
        .bind(&new_id)
        .bind(&module.title)
        .bind(&module.description)
        .bind(module.order_index)
        .bind(&module.color)
        .bind(&now)
        .bind(&now)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    }
    
    Ok(new_program)
}

#[tauri::command]
pub async fn get_program_stats(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<crate::db::models::ProgramStats, String> {
    let row: Option<(i32, i32, i32, f64, i32)> = sqlx::query_as(
        "SELECT 
            COALESCE(COUNT(DISTINCT dp.id), 0) as total_days,
            COALESCE(COUNT(DISTINCT CASE WHEN da.status IN ('passed', 'mastery') THEN da.day_plan_id END), 0) as completed_days,
            COALESCE(COUNT(DISTINCT CASE WHEN da.status = 'blocked' THEN da.day_plan_id END), 0) as blocked_days,
            COALESCE(AVG(CASE WHEN da.status IN ('passed', 'mastery') THEN da.total_score END), 0.0) as average_score,
            COALESCE(SUM(da.actual_minutes), 0) as total_time_minutes
         FROM programs p
         LEFT JOIN day_plans dp ON dp.program_id = p.id
         LEFT JOIN day_attempts da ON da.day_plan_id = dp.id
         WHERE p.id = ?
         GROUP BY p.id"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let stats = if let Some((total_days, completed_days, blocked_days, average_score, total_time_minutes)) = row {
        crate::db::models::ProgramStats {
            total_days,
            completed_days,
            blocked_days,
            average_score,
            current_streak: 0,
            total_time_minutes,
        }
    } else {
        crate::db::models::ProgramStats {
            total_days: 0,
            completed_days: 0,
            blocked_days: 0,
            average_score: 0.0,
            current_streak: 0,
            total_time_minutes: 0,
        }
    };
    
    Ok(stats)
}
