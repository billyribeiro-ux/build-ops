use crate::db::models::*;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

// Core Day Plan CRUD (7 commands)

#[tauri::command]
pub async fn create_day_plan(
    pool: State<'_, SqlitePool>,
    input: CreateDayPlanInput,
) -> Result<DayPlan, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get next day number
    let next_day_number: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(day_number), 0) + 1 FROM day_plans WHERE program_id = ?"
    )
    .bind(&input.program_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let day_plan = sqlx::query_as::<_, DayPlan>(
        "INSERT INTO day_plans (
            id, program_id, module_id, title, day_number, version, status,
            syntax_targets, implementation_brief, files_to_create, success_criteria,
            stretch_challenge, notes, estimated_minutes, memory_rebuild_minutes,
            min_minutes, recommended_minutes, deep_minutes, complexity_level,
            focus_blocks, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, 1, 'draft', ?, ?, '', '', '', '', ?, 15, ?, ?, ?, ?, '[]', ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.program_id)
    .bind(&input.module_id)
    .bind(&input.title)
    .bind(next_day_number)
    .bind(&input.syntax_targets)
    .bind(&input.implementation_brief)
    .bind(input.min_minutes.unwrap_or(60))
    .bind(input.min_minutes.unwrap_or(60))
    .bind(input.recommended_minutes.unwrap_or(90))
    .bind(input.deep_minutes.unwrap_or(120))
    .bind(input.complexity_level.unwrap_or(3))
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(day_plan)
}

#[tauri::command]
pub async fn get_day_plan(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DayPlanFull, String> {
    let day_plan = sqlx::query_as::<_, DayPlan>(
        "SELECT * FROM day_plans WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let module: (String, String) = sqlx::query_as(
        "SELECT title, color FROM modules WHERE id = ?"
    )
    .bind(&day_plan.module_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let checklist_items = sqlx::query_as::<_, ChecklistItem>(
        "SELECT * FROM checklist_items WHERE day_plan_id = ? ORDER BY order_index"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let quiz_questions = sqlx::query_as::<_, QuizQuestion>(
        "SELECT * FROM quiz_questions WHERE day_plan_id = ? ORDER BY order_index"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let concept_tags = sqlx::query_as::<_, ConceptTag>(
        "SELECT ct.* FROM concept_tags ct
         JOIN day_plan_tags dpt ON ct.id = dpt.concept_tag_id
         WHERE dpt.day_plan_id = ?
         ORDER BY ct.name"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let dependencies = sqlx::query_as::<_, DayDependency>(
        "SELECT * FROM day_dependencies WHERE day_plan_id = ?"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(DayPlanFull {
        day_plan,
        checklist_items,
        quiz_questions,
        concept_tags,
        dependencies,
        module_title: module.0,
        module_color: module.1,
    })
}

#[tauri::command]
pub async fn list_day_plans(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<DayPlanSummary>, String> {
    let summaries = sqlx::query_as::<_, DayPlanSummary>(
        "SELECT 
            dp.id, dp.title, dp.day_number, dp.module_id, dp.status, dp.estimated_minutes,
            m.title as module_title, m.color as module_color,
            COALESCE(COUNT(DISTINCT ci.id), 0) as checklist_count,
            COALESCE(COUNT(DISTINCT qq.id), 0) as quiz_count,
            COALESCE(COUNT(DISTINCT dpt.concept_tag_id), 0) as tag_count,
            MAX(da.total_score) as best_score,
            COALESCE(COUNT(DISTINCT da.id), 0) as attempt_count
         FROM day_plans dp
         JOIN modules m ON dp.module_id = m.id
         LEFT JOIN checklist_items ci ON dp.id = ci.day_plan_id
         LEFT JOIN quiz_questions qq ON dp.id = qq.day_plan_id
         LEFT JOIN day_plan_tags dpt ON dp.id = dpt.day_plan_id
         LEFT JOIN day_attempts da ON dp.id = da.day_plan_id
         WHERE dp.program_id = ?
         GROUP BY dp.id
         ORDER BY dp.day_number"
    )
    .bind(&program_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(summaries)
}

#[tauri::command]
pub async fn list_day_plans_by_module(
    pool: State<'_, SqlitePool>,
    module_id: String,
) -> Result<Vec<DayPlanSummary>, String> {
    let summaries = sqlx::query_as::<_, DayPlanSummary>(
        "SELECT 
            dp.id, dp.title, dp.day_number, dp.module_id, dp.status, dp.estimated_minutes,
            m.title as module_title, m.color as module_color,
            COALESCE(COUNT(DISTINCT ci.id), 0) as checklist_count,
            COALESCE(COUNT(DISTINCT qq.id), 0) as quiz_count,
            COALESCE(COUNT(DISTINCT dpt.concept_tag_id), 0) as tag_count,
            MAX(da.total_score) as best_score,
            COALESCE(COUNT(DISTINCT da.id), 0) as attempt_count
         FROM day_plans dp
         JOIN modules m ON dp.module_id = m.id
         LEFT JOIN checklist_items ci ON dp.id = ci.day_plan_id
         LEFT JOIN quiz_questions qq ON dp.id = qq.day_plan_id
         LEFT JOIN day_plan_tags dpt ON dp.id = dpt.day_plan_id
         LEFT JOIN day_attempts da ON dp.id = da.day_plan_id
         WHERE dp.module_id = ?
         GROUP BY dp.id
         ORDER BY dp.day_number"
    )
    .bind(&module_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(summaries)
}

#[tauri::command]
pub async fn update_day_plan(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateDayPlanInput,
) -> Result<DayPlan, String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get current day plan to check if published
    let current: DayPlan = sqlx::query_as("SELECT * FROM day_plans WHERE id = ?")
        .bind(&id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    let mut version = current.version;
    
    // If published and content changed, increment version
    if current.status == "published" && (
        input.syntax_targets.is_some() ||
        input.implementation_brief.is_some() ||
        input.files_to_create.is_some() ||
        input.success_criteria.is_some() ||
        input.stretch_challenge.is_some()
    ) {
        version += 1;
    }
    
    let day_plan = sqlx::query_as::<_, DayPlan>(
        "UPDATE day_plans SET
            title = COALESCE(?, title),
            syntax_targets = COALESCE(?, syntax_targets),
            implementation_brief = COALESCE(?, implementation_brief),
            files_to_create = COALESCE(?, files_to_create),
            success_criteria = COALESCE(?, success_criteria),
            stretch_challenge = COALESCE(?, stretch_challenge),
            notes = COALESCE(?, notes),
            status = COALESCE(?, status),
            estimated_minutes = COALESCE(?, estimated_minutes),
            memory_rebuild_minutes = COALESCE(?, memory_rebuild_minutes),
            version = ?,
            updated_at = ?
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.title.as_ref())
    .bind(input.syntax_targets.as_ref())
    .bind(input.implementation_brief.as_ref())
    .bind(input.files_to_create.as_ref())
    .bind(input.success_criteria.as_ref())
    .bind(input.stretch_challenge.as_ref())
    .bind(input.notes.as_ref())
    .bind(input.status.as_ref())
    .bind(input.estimated_minutes)
    .bind(input.memory_rebuild_minutes)
    .bind(version)
    .bind(&now)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(day_plan)
}

#[tauri::command]
pub async fn delete_day_plan(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    // Get program_id before deleting
    let program_id: String = sqlx::query_scalar(
        "SELECT program_id FROM day_plans WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    // Delete day plan (cascades to checklist, quiz, tags, dependencies)
    sqlx::query("DELETE FROM day_plans WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    // Resequence day numbers
    sqlx::query(
        "UPDATE day_plans SET day_number = (
            SELECT COUNT(*) FROM day_plans dp2
            WHERE dp2.program_id = day_plans.program_id
            AND dp2.day_number <= day_plans.day_number
        )
        WHERE program_id = ?"
    )
    .bind(&program_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn reorder_day_plans(
    pool: State<'_, SqlitePool>,
    day_plan_ids: Vec<String>,
) -> Result<(), String> {
    for (index, day_plan_id) in day_plan_ids.iter().enumerate() {
        sqlx::query("UPDATE day_plans SET day_number = ? WHERE id = ?")
            .bind((index + 1) as i32)
            .bind(day_plan_id)
            .execute(pool.inner())
            .await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn duplicate_day_plan(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<DayPlan, String> {
    let original = sqlx::query_as::<_, DayPlan>(
        "SELECT * FROM day_plans WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let new_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get next day number
    let next_day_number: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(day_number), 0) + 1 FROM day_plans WHERE program_id = ?"
    )
    .bind(&original.program_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    // Create new day plan
    let new_plan = sqlx::query_as::<_, DayPlan>(
        "INSERT INTO day_plans (
            id, program_id, module_id, title, day_number, version, status,
            syntax_targets, implementation_brief, files_to_create, success_criteria,
            stretch_challenge, notes, estimated_minutes, memory_rebuild_minutes,
            min_minutes, recommended_minutes, deep_minutes, complexity_level,
            focus_blocks, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, 1, 'draft', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&new_id)
    .bind(&original.program_id)
    .bind(&original.module_id)
    .bind(format!("{} (Copy)", original.title))
    .bind(next_day_number)
    .bind(&original.syntax_targets)
    .bind(&original.implementation_brief)
    .bind(&original.files_to_create)
    .bind(&original.success_criteria)
    .bind(&original.stretch_challenge)
    .bind(&original.notes)
    .bind(original.estimated_minutes)
    .bind(original.memory_rebuild_minutes)
    .bind(original.min_minutes)
    .bind(original.recommended_minutes)
    .bind(original.deep_minutes)
    .bind(original.complexity_level)
    .bind(&original.focus_blocks)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    // Copy checklist items
    let checklist_items = sqlx::query_as::<_, ChecklistItem>(
        "SELECT * FROM checklist_items WHERE day_plan_id = ?"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    for item in checklist_items {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO checklist_items (id, day_plan_id, label, is_required, order_index, created_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&item_id)
        .bind(&new_id)
        .bind(&item.label)
        .bind(item.is_required)
        .bind(item.order_index)
        .bind(&now)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    }
    
    // Copy quiz questions
    let quiz_questions = sqlx::query_as::<_, QuizQuestion>(
        "SELECT * FROM quiz_questions WHERE day_plan_id = ?"
    )
    .bind(&id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    for question in quiz_questions {
        let question_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO quiz_questions (
                id, day_plan_id, question_text, question_type, correct_answer,
                options, points, time_limit_seconds, order_index, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&question_id)
        .bind(&new_id)
        .bind(&question.question_text)
        .bind(&question.question_type)
        .bind(&question.correct_answer)
        .bind(&question.options)
        .bind(question.points)
        .bind(question.time_limit_seconds)
        .bind(question.order_index)
        .bind(&now)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    }
    
    Ok(new_plan)
}

// Checklist Management (4 commands)

#[tauri::command]
pub async fn add_checklist_item(
    pool: State<'_, SqlitePool>,
    input: CreateChecklistItemInput,
) -> Result<ChecklistItem, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get next order index
    let next_order: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(order_index), -1) + 1 FROM checklist_items WHERE day_plan_id = ?"
    )
    .bind(&input.day_plan_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let item = sqlx::query_as::<_, ChecklistItem>(
        "INSERT INTO checklist_items (id, day_plan_id, label, is_required, order_index, created_at)
         VALUES (?, ?, ?, ?, ?, ?)
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_plan_id)
    .bind(&input.label)
    .bind(input.is_required)
    .bind(next_order)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(item)
}

#[tauri::command]
pub async fn update_checklist_item(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateChecklistItemInput,
) -> Result<ChecklistItem, String> {
    let item = sqlx::query_as::<_, ChecklistItem>(
        "UPDATE checklist_items SET
            label = COALESCE(?, label),
            is_required = COALESCE(?, is_required)
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.label.as_ref())
    .bind(input.is_required)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(item)
}

#[tauri::command]
pub async fn delete_checklist_item(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    // Get day_plan_id before deleting
    let day_plan_id: String = sqlx::query_scalar(
        "SELECT day_plan_id FROM checklist_items WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    sqlx::query("DELETE FROM checklist_items WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    // Resequence order indices
    sqlx::query(
        "UPDATE checklist_items SET order_index = (
            SELECT COUNT(*) FROM checklist_items ci2
            WHERE ci2.day_plan_id = checklist_items.day_plan_id
            AND ci2.order_index <= checklist_items.order_index
        ) - 1
        WHERE day_plan_id = ?"
    )
    .bind(&day_plan_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn reorder_checklist_items(
    pool: State<'_, SqlitePool>,
    item_ids: Vec<String>,
) -> Result<(), String> {
    for (index, item_id) in item_ids.iter().enumerate() {
        sqlx::query("UPDATE checklist_items SET order_index = ? WHERE id = ?")
            .bind(index as i32)
            .bind(item_id)
            .execute(pool.inner())
            .await
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

// Quiz Management (3 commands)

#[tauri::command]
pub async fn add_quiz_question(
    pool: State<'_, SqlitePool>,
    input: CreateQuizQuestionInput,
) -> Result<QuizQuestion, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get next order index
    let next_order: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(order_index), -1) + 1 FROM quiz_questions WHERE day_plan_id = ?"
    )
    .bind(&input.day_plan_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let question = sqlx::query_as::<_, QuizQuestion>(
        "INSERT INTO quiz_questions (
            id, day_plan_id, question_text, question_type, correct_answer,
            options, points, time_limit_seconds, order_index, created_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_plan_id)
    .bind(&input.question_text)
    .bind(&input.question_type)
    .bind(&input.correct_answer)
    .bind(input.options.unwrap_or_else(|| "[]".to_string()))
    .bind(input.points)
    .bind(input.time_limit_seconds)
    .bind(next_order)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(question)
}

#[tauri::command]
pub async fn update_quiz_question(
    pool: State<'_, SqlitePool>,
    id: String,
    input: UpdateQuizQuestionInput,
) -> Result<QuizQuestion, String> {
    let question = sqlx::query_as::<_, QuizQuestion>(
        "UPDATE quiz_questions SET
            question_text = COALESCE(?, question_text),
            question_type = COALESCE(?, question_type),
            correct_answer = COALESCE(?, correct_answer),
            options = COALESCE(?, options),
            points = COALESCE(?, points),
            time_limit_seconds = COALESCE(?, time_limit_seconds)
         WHERE id = ?
         RETURNING *"
    )
    .bind(input.question_text.as_ref())
    .bind(input.question_type.as_ref())
    .bind(input.correct_answer.as_ref())
    .bind(input.options.as_ref())
    .bind(input.points)
    .bind(input.time_limit_seconds)
    .bind(&id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(question)
}

#[tauri::command]
pub async fn delete_quiz_question(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM quiz_questions WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Concept Tags (4 commands)

#[tauri::command]
pub async fn create_concept_tag(
    pool: State<'_, SqlitePool>,
    input: CreateConceptTagInput,
) -> Result<ConceptTag, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let tag = sqlx::query_as::<_, ConceptTag>(
        "INSERT INTO concept_tags (id, name, domain, color, created_at)
         VALUES (?, ?, ?, ?, ?)
         RETURNING *"
    )
    .bind(&id)
    .bind(&input.name)
    .bind(&input.domain)
    .bind(&input.color)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(tag)
}

#[tauri::command]
pub async fn list_concept_tags(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<ConceptTag>, String> {
    let tags = sqlx::query_as::<_, ConceptTag>(
        "SELECT * FROM concept_tags ORDER BY domain, name"
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(tags)
}

#[tauri::command]
pub async fn add_tag_to_day(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
    concept_tag_id: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    // Idempotent - ignore if already exists
    sqlx::query(
        "INSERT OR IGNORE INTO day_plan_tags (day_plan_id, concept_tag_id, created_at)
         VALUES (?, ?, ?)"
    )
    .bind(&day_plan_id)
    .bind(&concept_tag_id)
    .bind(&now)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn remove_tag_from_day(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
    concept_tag_id: String,
) -> Result<(), String> {
    sqlx::query(
        "DELETE FROM day_plan_tags WHERE day_plan_id = ? AND concept_tag_id = ?"
    )
    .bind(&day_plan_id)
    .bind(&concept_tag_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

// Dependencies (3 commands)

#[tauri::command]
pub async fn add_dependency(
    pool: State<'_, SqlitePool>,
    input: CreateDependencyInput,
) -> Result<DayDependency, String> {
    // Check for circular dependencies
    let is_circular = check_circular_dependency(
        pool.inner(),
        &input.day_plan_id,
        &input.depends_on_day_plan_id
    ).await?;
    
    if is_circular {
        return Err("Circular dependency detected".to_string());
    }
    
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    let dependency = sqlx::query_as::<_, DayDependency>(
        "INSERT INTO day_dependencies (
            id, day_plan_id, depends_on_day_plan_id, dependency_type, minimum_score, created_at
        ) VALUES (?, ?, ?, ?, ?, ?)
        RETURNING *"
    )
    .bind(&id)
    .bind(&input.day_plan_id)
    .bind(&input.depends_on_day_plan_id)
    .bind(&input.dependency_type)
    .bind(input.minimum_score)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(dependency)
}

#[tauri::command]
pub async fn remove_dependency(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<(), String> {
    sqlx::query("DELETE FROM day_dependencies WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn check_dependencies(
    pool: State<'_, SqlitePool>,
    day_plan_id: String,
) -> Result<Vec<DependencyStatus>, String> {
    let dependencies = sqlx::query_as::<_, DayDependency>(
        "SELECT * FROM day_dependencies WHERE day_plan_id = ?"
    )
    .bind(&day_plan_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let mut statuses = Vec::new();
    
    for dep in dependencies {
        let day_info: (String, i32) = sqlx::query_as(
            "SELECT title, day_number FROM day_plans WHERE id = ?"
        )
        .bind(&dep.depends_on_day_plan_id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
        
        let best_attempt: Option<(i32, String)> = sqlx::query_as(
            "SELECT total_score, status FROM day_attempts
             WHERE day_plan_id = ? AND is_draft = 0
             ORDER BY total_score DESC LIMIT 1"
        )
        .bind(&dep.depends_on_day_plan_id)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
        
        let (best_score, best_status) = match best_attempt {
            Some((score, status)) => (Some(score), Some(status)),
            None => (None, None),
        };
        
        let is_met = match dep.dependency_type.as_str() {
            "prerequisite" => best_score.map_or(false, |s| s >= dep.minimum_score),
            _ => true, // recommended and related don't block
        };
        
        statuses.push(DependencyStatus {
            dependency_id: dep.id,
            depends_on_day_plan_id: dep.depends_on_day_plan_id,
            depends_on_title: day_info.0,
            depends_on_day_number: day_info.1,
            dependency_type: dep.dependency_type,
            minimum_score: dep.minimum_score,
            is_met,
            best_score,
            best_attempt_status: best_status,
        });
    }
    
    Ok(statuses)
}

// Helper function for circular dependency detection
async fn check_circular_dependency(
    pool: &SqlitePool,
    day_plan_id: &str,
    depends_on_id: &str,
) -> Result<bool, String> {
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![depends_on_id.to_string()];
    
    while let Some(current) = stack.pop() {
        if current == day_plan_id {
            return Ok(true); // Circular!
        }
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        
        let deps: Vec<String> = sqlx::query_scalar(
            "SELECT depends_on_day_plan_id FROM day_dependencies WHERE day_plan_id = ?"
        )
        .bind(&current)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;
        
        stack.extend(deps);
    }
    
    Ok(false)
}
