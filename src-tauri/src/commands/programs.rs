use crate::db::models::{Program, CreateProgramInput, UpdateProgramInput};
use crate::error::Result;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_program(
    pool: State<'_, SqlitePool>,
    input: CreateProgramInput,
) -> Result<Program> {
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
    .await?;
    
    Ok(program)
}

#[tauri::command]
pub async fn get_program(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<Program> {
    let program = sqlx::query_as::<_, Program>(
        "SELECT * FROM programs WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await?
    .ok_or_else(|| crate::error::AppError::NotFound(format!("Program {} not found", id)))?;
    
    Ok(program)
}

#[tauri::command]
pub async fn list_programs(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Program>> {
    let programs = sqlx::query_as::<_, Program>(
        "SELECT * FROM programs ORDER BY created_at DESC"
    )
    .fetch_all(pool.inner())
    .await?;
    
    Ok(programs)
}

#[tauri::command]
pub async fn update_program(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateProgramInput,
) -> Result<Program> {
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
    
    let program = q.fetch_optional(pool.inner()).await?
        .ok_or_else(|| crate::error::AppError::NotFound(format!("Program {} not found", id)))?;
    
    Ok(program)
}

#[tauri::command]
pub async fn delete_program(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<()> {
    sqlx::query("DELETE FROM programs WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await?;
    
    Ok(())
}
