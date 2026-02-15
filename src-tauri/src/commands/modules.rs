use crate::db::models::{Module, CreateModuleInput, UpdateModuleInput};
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn create_module(
    pool: State<'_, SqlitePool>,
    input: CreateModuleInput,
) -> Result<Module, String> {
    let id = Uuid::new_v4().to_string();
    
    // Get the next order_index for this program
    let max_order: Option<i32> = sqlx::query_scalar(
        "SELECT MAX(order_index) FROM modules WHERE program_id = ?"
    )
    .bind(&input.program_id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let order_index = max_order.map_or(0, |max| max + 1);
    
    let module = sqlx::query_as::<_, Module>(
        "INSERT INTO modules (id, program_id, title, description, color, order_index) 
         VALUES (?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.program_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.color)
    .bind(order_index)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(module)
}

#[tauri::command]
pub async fn get_module(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<Module, String> {
    let module = sqlx::query_as::<_, Module>(
        "SELECT * FROM modules WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| e.to_string())?
    .ok_or_else(|| format!("Module {} not found", id))?;
    
    Ok(module)
}

#[tauri::command]
pub async fn list_modules(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<Module>, String> {
    let modules = sqlx::query_as::<_, Module>(
        "SELECT * FROM modules WHERE program_id = ? ORDER BY order_index"
    )
    .bind(&program_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(modules)
}

#[tauri::command]
pub async fn update_module(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateModuleInput,
) -> Result<Module, String> {
    let mut query = String::from("UPDATE modules SET updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')");
    let mut params: Vec<String> = Vec::new();
    
    if let Some(title) = input.title {
        query.push_str(", title = ?");
        params.push(title);
    }
    if let Some(description) = input.description {
        query.push_str(", description = ?");
        params.push(description);
    }
    if let Some(color) = input.color {
        query.push_str(", color = ?");
        params.push(color);
    }
    
    query.push_str(" WHERE id = ? RETURNING *");
    params.push(id.clone());
    
    let mut q = sqlx::query_as::<_, Module>(&query);
    for param in params {
        q = q.bind(param);
    }
    
    let module = q.fetch_optional(pool.inner()).await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Module {} not found", id))?;
    
    Ok(module)
}

#[tauri::command]
pub async fn delete_module(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM modules WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn reorder_modules(
    pool: State<'_, SqlitePool>,
    program_id: String,
    module_ids: Vec<String>,
) -> Result<(), String> {
    // Update order_index for each module based on position in the array
    for (index, module_id) in module_ids.iter().enumerate() {
        sqlx::query(
            "UPDATE modules SET order_index = ? WHERE id = ? AND program_id = ?"
        )
        .bind(index as i32)
        .bind(module_id)
        .bind(&program_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
