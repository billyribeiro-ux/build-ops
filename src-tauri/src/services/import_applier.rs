use crate::db::models::import::GeneratedPlan;
use crate::db::models::Program;
use crate::error::AppError;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

pub async fn apply_import(
    pool: &Pool<Sqlite>,
    plan: &GeneratedPlan,
    existing_program_id: Option<String>,
) -> Result<Program, AppError> {
    let mut tx = pool.begin().await?;

    let program_id = if let Some(id) = existing_program_id {
        sqlx::query(
            "UPDATE programs SET title = ?, description = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(&plan.program.title)
        .bind(&plan.program.description)
        .bind(&id)
        .execute(&mut *tx)
        .await?;
        id
    } else {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO programs (id, title, description, target_days, status) VALUES (?, ?, ?, ?, 'active')",
        )
        .bind(&id)
        .bind(&plan.program.title)
        .bind(&plan.program.description)
        .bind(plan.program.estimated_total_days)
        .execute(&mut *tx)
        .await?;
        id
    };

    let mut module_ids = Vec::new();
    for module in &plan.modules {
        let module_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO modules (id, program_id, title, description, order_index, color) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&module_id)
        .bind(&program_id)
        .bind(&module.title)
        .bind(&module.description)
        .bind(module.order_index)
        .bind(&module.color)
        .execute(&mut *tx)
        .await?;
        module_ids.push(module_id);
    }

    let mut day_plan_ids = Vec::new();
    for day in &plan.day_plans {
        let day_id = Uuid::new_v4().to_string();
        let module_id = &module_ids[day.module_index];

        let focus_blocks = serde_json::json!([
            {"session_type": "learn", "minutes": day.estimated_minutes / 3},
            {"session_type": "build", "minutes": day.estimated_minutes / 2},
            {"session_type": "review", "minutes": day.estimated_minutes / 6}
        ])
        .to_string();

        sqlx::query(
            "INSERT INTO day_plans (id, program_id, module_id, title, day_number, version, status, \
             syntax_targets, implementation_brief, files_to_create, success_criteria, stretch_challenge, \
             notes, estimated_minutes, memory_rebuild_minutes, min_minutes, recommended_minutes, \
             deep_minutes, complexity_level, focus_blocks) \
             VALUES (?, ?, ?, ?, ?, 1, 'published', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&day_id)
        .bind(&program_id)
        .bind(module_id)
        .bind(&day.title)
        .bind(day.day_number)
        .bind(&day.syntax_targets)
        .bind(&day.implementation_brief)
        .bind(&day.files_to_create)
        .bind(&day.success_criteria)
        .bind(&day.stretch_challenge)
        .bind(&day.notes)
        .bind(day.estimated_minutes)
        .bind(day.memory_rebuild_minutes)
        .bind(day.min_minutes)
        .bind(day.recommended_minutes)
        .bind(day.deep_minutes)
        .bind(day.complexity_level)
        .bind(&focus_blocks)
        .execute(&mut *tx)
        .await?;
        day_plan_ids.push(day_id);
    }

    for item in &plan.checklist_items {
        let item_id = Uuid::new_v4().to_string();
        let day_id = &day_plan_ids[item.day_index];

        sqlx::query(
            "INSERT INTO checklist_items (id, day_plan_id, label, is_required, order_index) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&item_id)
        .bind(day_id)
        .bind(&item.label)
        .bind(if item.is_required { 1 } else { 0 })
        .bind(item.order_index)
        .execute(&mut *tx)
        .await?;
    }

    for question in &plan.quiz_questions {
        let question_id = Uuid::new_v4().to_string();
        let day_id = &day_plan_ids[question.day_index];

        let options_json = serde_json::to_string(&question.options).unwrap_or_default();

        sqlx::query(
            "INSERT INTO quiz_questions (id, day_plan_id, question_text, question_type, \
             correct_answer, options_json, points, time_limit_seconds, order_index) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0)",
        )
        .bind(&question_id)
        .bind(day_id)
        .bind(&question.question_text)
        .bind(&question.question_type)
        .bind(&question.correct_answer)
        .bind(&options_json)
        .bind(question.points)
        .bind(question.time_limit_seconds)
        .execute(&mut *tx)
        .await?;
    }

    let mut tag_id_map = std::collections::HashMap::new();
    for tag in &plan.concept_tags {
        let existing: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM concept_tags WHERE name = ? AND domain = ?",
        )
        .bind(&tag.name)
        .bind(&tag.domain)
        .fetch_optional(&mut *tx)
        .await?;

        let tag_id = if let Some((id,)) = existing {
            id
        } else {
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO concept_tags (id, name, domain, color) VALUES (?, ?, ?, '#6366F1')",
            )
            .bind(&id)
            .bind(&tag.name)
            .bind(&tag.domain)
            .execute(&mut *tx)
            .await?;
            id
        };

        tag_id_map.insert(tag.name.clone(), tag_id);
    }

    for (day_index, tag_name) in &plan.tag_assignments {
        if let Some(tag_id) = tag_id_map.get(tag_name) {
            let day_id = &day_plan_ids[*day_index];
            sqlx::query(
                "INSERT INTO day_plan_tags (day_plan_id, tag_id) VALUES (?, ?) ON CONFLICT DO NOTHING",
            )
            .bind(day_id)
            .bind(tag_id)
            .execute(&mut *tx)
            .await?;
        }
    }

    for dep in &plan.dependencies {
        let day_id = &day_plan_ids[dep.day_index];
        
        let depends_on_day = plan.day_plans
            .iter()
            .find(|d| d.day_number == dep.depends_on_day_number);

        if let Some(depends_on) = depends_on_day {
            let depends_on_id = &day_plan_ids[plan.day_plans.iter().position(|d| d.day_number == depends_on.day_number).unwrap()];

            let dep_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO day_dependencies (id, day_plan_id, depends_on_day_plan_id, dependency_type, minimum_score) \
                 VALUES (?, ?, ?, ?, ?)",
            )
            .bind(&dep_id)
            .bind(day_id)
            .bind(depends_on_id)
            .bind(&dep.dependency_type)
            .bind(dep.minimum_score)
            .execute(&mut *tx)
            .await?;
        }
    }

    tx.commit().await?;

    let program: Program = sqlx::query_as("SELECT * FROM programs WHERE id = ?")
        .bind(&program_id)
        .fetch_one(pool)
        .await?;

    Ok(program)
}
