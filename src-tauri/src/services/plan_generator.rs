use crate::db::models::import::{
    AiAnalysisResponse, ChecklistItemDraft, ConceptTagDraft, DayPlanDraft, DependencyDraft,
    GeneratedPlan, ModuleDraft, ProgramDraft, QuizQuestionDraft,
};
use crate::error::AppError;
use std::collections::{HashMap, HashSet};

pub fn generate_plan(ai_response: AiAnalysisResponse) -> Result<GeneratedPlan, AppError> {
    let mut validation_warnings = Vec::new();

    let program = ProgramDraft {
        title: ai_response.program_title,
        description: ai_response.program_description,
        estimated_total_days: ai_response.estimated_total_days,
    };

    let mut modules = Vec::new();
    let mut day_plans = Vec::new();
    let mut checklist_items = Vec::new();
    let mut quiz_questions = Vec::new();
    let mut concept_tags_set = HashSet::new();
    let mut tag_assignments = Vec::new();
    let mut dependencies = Vec::new();

    let mut global_day_index = 0;
    let mut day_number_to_index = HashMap::new();

    for (module_idx, ai_module) in ai_response.modules.iter().enumerate() {
        modules.push(ModuleDraft {
            title: ai_module.title.clone(),
            description: ai_module.description.clone(),
            order_index: ai_module.order_index,
            color: validate_color(&ai_module.color),
        });

        for ai_day in &ai_module.days {
            let day_number = ai_day.day_number;
            day_number_to_index.insert(day_number, global_day_index);

            let estimated_minutes = clamp_minutes(ai_day.estimated_minutes, &mut validation_warnings);
            let memory_rebuild_minutes = clamp_minutes(ai_day.memory_rebuild_minutes, &mut validation_warnings);

            let complexity_level = calculate_complexity(
                &ai_day.syntax_targets,
                &ai_day.implementation_brief,
                estimated_minutes,
            );

            day_plans.push(DayPlanDraft {
                module_index: module_idx,
                day_number,
                title: ai_day.title.clone(),
                syntax_targets: ai_day.syntax_targets.clone(),
                implementation_brief: ai_day.implementation_brief.clone(),
                files_to_create: ai_day.files_to_create.clone(),
                success_criteria: ai_day.success_criteria.clone(),
                stretch_challenge: ai_day.stretch_challenge.clone(),
                notes: ai_day.notes.clone(),
                estimated_minutes,
                memory_rebuild_minutes,
                min_minutes: (estimated_minutes as f64 * 0.75) as i64,
                recommended_minutes: estimated_minutes,
                deep_minutes: (estimated_minutes as f64 * 1.5) as i64,
                complexity_level,
            });

            if ai_day.checklist_items.is_empty() {
                validation_warnings.push(format!(
                    "Day {} has no checklist items, adding default",
                    day_number
                ));
                checklist_items.push(ChecklistItemDraft {
                    day_index: global_day_index,
                    label: "Complete the implementation".to_string(),
                    is_required: true,
                    order_index: 0,
                });
            } else {
                for (idx, item) in ai_day.checklist_items.iter().enumerate() {
                    checklist_items.push(ChecklistItemDraft {
                        day_index: global_day_index,
                        label: item.label.clone(),
                        is_required: item.is_required,
                        order_index: idx as i64,
                    });
                }
            }

            if ai_day.quiz_questions.is_empty() {
                validation_warnings.push(format!(
                    "Day {} has no quiz questions, adding default",
                    day_number
                ));
                quiz_questions.push(QuizQuestionDraft {
                    day_index: global_day_index,
                    question_text: format!("What did you learn in {}?", ai_day.title),
                    question_type: "reflection".to_string(),
                    correct_answer: String::new(),
                    options: Vec::new(),
                    points: 10,
                    time_limit_seconds: 300,
                });
            } else {
                for question in &ai_day.quiz_questions {
                    quiz_questions.push(QuizQuestionDraft {
                        day_index: global_day_index,
                        question_text: question.question_text.clone(),
                        question_type: validate_question_type(&question.question_type),
                        correct_answer: question.correct_answer.clone(),
                        options: question.options.clone(),
                        points: question.points.max(1),
                        time_limit_seconds: question.time_limit_seconds.max(30),
                    });
                }
            }

            for tag in &ai_day.concept_tags {
                let normalized_name = normalize_tag_name(&tag.name);
                concept_tags_set.insert((normalized_name.clone(), tag.domain.clone()));
                tag_assignments.push((global_day_index, normalized_name));
            }

            for dep in &ai_day.dependencies {
                dependencies.push(DependencyDraft {
                    day_index: global_day_index,
                    depends_on_day_number: dep.depends_on_day_number,
                    dependency_type: validate_dependency_type(&dep.dependency_type),
                    minimum_score: dep.minimum_score.max(0).min(100),
                });
            }

            global_day_index += 1;
        }
    }

    validate_dependencies(&dependencies, &day_number_to_index, &mut validation_warnings);

    let concept_tags: Vec<ConceptTagDraft> = concept_tags_set
        .into_iter()
        .map(|(name, domain)| ConceptTagDraft { name, domain })
        .collect();

    Ok(GeneratedPlan {
        program,
        modules,
        day_plans,
        checklist_items,
        quiz_questions,
        concept_tags,
        tag_assignments,
        dependencies,
        validation_warnings,
    })
}

fn clamp_minutes(minutes: i64, warnings: &mut Vec<String>) -> i64 {
    if minutes < 15 {
        warnings.push(format!("Minutes {} too low, clamping to 45", minutes));
        45
    } else if minutes > 180 {
        warnings.push(format!("Minutes {} too high, clamping to 180", minutes));
        180
    } else {
        minutes
    }
}

fn validate_color(color: &str) -> String {
    let valid_colors = [
        "#6366F1", "#EC4899", "#F59E0B", "#22C55E", "#3B82F6", "#A855F7", "#EF4444", "#14B8A6",
    ];

    if valid_colors.contains(&color) {
        color.to_string()
    } else {
        "#6366F1".to_string()
    }
}

fn calculate_complexity(syntax: &str, implementation: &str, minutes: i64) -> i64 {
    let mut score = 1i64;

    if minutes > 90 {
        score += 1;
    }
    if minutes > 120 {
        score += 1;
    }

    let combined = format!("{} {}", syntax, implementation).to_lowercase();
    let complex_keywords = [
        "async",
        "await",
        "closure",
        "generic",
        "trait",
        "interface",
        "algorithm",
        "architecture",
        "state management",
        "optimization",
    ];

    for keyword in &complex_keywords {
        if combined.contains(keyword) {
            score += 1;
            break;
        }
    }

    if syntax.len() > 500 || implementation.len() > 1000 {
        score += 1;
    }

    score.min(5)
}

fn validate_question_type(qtype: &str) -> String {
    match qtype {
        "short_answer" | "multiple_choice" | "code_prompt" | "reflection" => qtype.to_string(),
        _ => "short_answer".to_string(),
    }
}

fn validate_dependency_type(dtype: &str) -> String {
    match dtype {
        "prerequisite" | "recommended" => dtype.to_string(),
        _ => "prerequisite".to_string(),
    }
}

fn normalize_tag_name(name: &str) -> String {
    name.trim().to_lowercase()
}

fn validate_dependencies(
    dependencies: &[DependencyDraft],
    day_map: &HashMap<i64, usize>,
    warnings: &mut Vec<String>,
) {
    for dep in dependencies {
        if !day_map.contains_key(&dep.depends_on_day_number) {
            warnings.push(format!(
                "Dependency references non-existent day {}",
                dep.depends_on_day_number
            ));
        }

        if let Some(&dep_idx) = day_map.get(&dep.depends_on_day_number) {
            if dep_idx >= dep.day_index {
                warnings.push(format!(
                    "Circular or forward dependency detected: day at index {} depends on day {}",
                    dep.day_index, dep.depends_on_day_number
                ));
            }
        }
    }
}
