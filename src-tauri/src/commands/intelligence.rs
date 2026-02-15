use crate::db::models::*;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

// Spaced Repetition Commands (3 commands)

#[tauri::command]
pub async fn get_due_reviews(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<DueReview>, String> {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    let reviews = sqlx::query_as::<_, DueReview>(
        "SELECT 
            sr.id, sr.day_plan_id, dp.title as day_title, dp.day_number,
            sr.concept_tag_id, ct.name as concept_name,
            sr.next_review_date, sr.interval_days
         FROM spaced_repetition sr
         JOIN day_plans dp ON sr.day_plan_id = dp.id
         JOIN concept_tags ct ON sr.concept_tag_id = ct.id
         WHERE dp.program_id = ? AND sr.next_review_date <= ?
         ORDER BY sr.next_review_date"
    )
    .bind(&program_id)
    .bind(&today)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(reviews)
}

#[tauri::command]
pub async fn record_review(
    pool: State<'_, SqlitePool>,
    input: RecordReviewInput,
) -> Result<SpacedRepetition, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    // Get existing spaced repetition entry or create new one
    let existing = sqlx::query_as::<_, SpacedRepetition>(
        "SELECT * FROM spaced_repetition 
         WHERE day_plan_id = ? AND concept_tag_id = ?"
    )
    .bind(&input.day_plan_id)
    .bind(&input.concept_tag_id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    let (easiness_factor, interval_days, repetition_count) = if let Some(sr) = existing {
        // SM-2 algorithm
        let quality = map_score_to_quality(input.score);
        let new_ef = calculate_easiness_factor(sr.easiness_factor, quality);
        let new_interval = calculate_interval(sr.interval_days, sr.repetition_count, quality);
        let new_count = if quality >= 3 { sr.repetition_count + 1 } else { 0 };
        (new_ef, new_interval, new_count)
    } else {
        // First review
        (2.5, 1, 1)
    };
    
    let next_review = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(interval_days as i64))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    
    let sr = sqlx::query_as::<_, SpacedRepetition>(
        "INSERT INTO spaced_repetition (
            id, day_plan_id, concept_tag_id, easiness_factor, interval_days,
            repetition_count, last_review_date, next_review_date, last_score,
            created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(day_plan_id, concept_tag_id) DO UPDATE SET
            easiness_factor = excluded.easiness_factor,
            interval_days = excluded.interval_days,
            repetition_count = excluded.repetition_count,
            last_review_date = excluded.last_review_date,
            next_review_date = excluded.next_review_date,
            last_score = excluded.last_score,
            updated_at = excluded.updated_at
        RETURNING *"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&input.day_plan_id)
    .bind(&input.concept_tag_id)
    .bind(easiness_factor)
    .bind(interval_days)
    .bind(repetition_count)
    .bind(&today)
    .bind(&next_review)
    .bind(input.score)
    .bind(&now)
    .bind(&now)
    .fetch_one(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(sr)
}

#[tauri::command]
pub async fn get_forgetting_curve_alerts(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<DueReview>, String> {
    let today = chrono::Utc::now();
    let threshold_date = today
        .checked_sub_signed(chrono::Duration::days(14))
        .unwrap()
        .format("%Y-%m-%d")
        .to_string();
    
    let alerts = sqlx::query_as::<_, DueReview>(
        "SELECT 
            sr.id, sr.day_plan_id, dp.title as day_title, dp.day_number,
            sr.concept_tag_id, ct.name as concept_name,
            sr.next_review_date, sr.interval_days
         FROM spaced_repetition sr
         JOIN day_plans dp ON sr.day_plan_id = dp.id
         JOIN concept_tags ct ON sr.concept_tag_id = ct.id
         WHERE dp.program_id = ? 
           AND sr.last_review_date < ?
           AND sr.last_score < 95
         ORDER BY sr.last_review_date"
    )
    .bind(&program_id)
    .bind(&threshold_date)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(alerts)
}

// Streak Commands (3 commands)

#[tauri::command]
pub async fn get_streak(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Streak, String> {
    let streak = sqlx::query_as::<_, Streak>(
        "SELECT * FROM streaks WHERE program_id = ?"
    )
    .bind(&program_id)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    if let Some(s) = streak {
        Ok(s)
    } else {
        // Create initial streak
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
        
        let new_streak = sqlx::query_as::<_, Streak>(
            "INSERT INTO streaks (
                id, program_id, current_streak, longest_streak,
                last_activity_date, freezes_available, freezes_used_this_month,
                created_at, updated_at
            ) VALUES (?, ?, 0, 0, ?, 2, 0, ?, ?)
            RETURNING *"
        )
        .bind(&id)
        .bind(&program_id)
        .bind(&today)
        .bind(&now)
        .bind(&now)
        .fetch_one(pool.inner())
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;
        
        Ok(new_streak)
    }
}

#[tauri::command]
pub async fn update_streak(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Streak, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    let current_streak = get_streak(pool.clone(), program_id.clone()).await?;
    
    let last_date = chrono::NaiveDate::parse_from_str(&current_streak.last_activity_date, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;
    let today_date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;
    
    let days_diff = (today_date - last_date).num_days();
    
    let new_streak = if days_diff == 0 {
        // Same day, no change
        current_streak.current_streak
    } else if days_diff == 1 {
        // Consecutive day, increment
        current_streak.current_streak + 1
    } else {
        // Gap, reset to 1
        1
    };
    
    let new_longest = new_streak.max(current_streak.longest_streak);
    
    let updated = sqlx::query_as::<_, Streak>(
        "UPDATE streaks SET
            current_streak = ?,
            longest_streak = ?,
            last_activity_date = ?,
            updated_at = ?
         WHERE program_id = ?
         RETURNING *"
    )
    .bind(new_streak)
    .bind(new_longest)
    .bind(&today)
    .bind(&now)
    .bind(&program_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(updated)
}

#[tauri::command]
pub async fn use_streak_freeze(
    pool: State<'_, SqlitePool>,
    program_id: String,
    reason: String,
) -> Result<Streak, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    
    let current_streak = get_streak(pool.clone(), program_id.clone()).await?;
    
    if current_streak.freezes_available <= 0 {
        return Err("No freezes available".to_string());
    }
    
    // Record freeze usage
    sqlx::query(
        "INSERT INTO streak_freezes (id, program_id, used_date, reason, created_at)
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&program_id)
    .bind(&today)
    .bind(&reason)
    .bind(&now)
    .execute(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    // Update streak
    let updated = sqlx::query_as::<_, Streak>(
        "UPDATE streaks SET
            freezes_available = freezes_available - 1,
            freezes_used_this_month = freezes_used_this_month + 1,
            updated_at = ?
         WHERE program_id = ?
         RETURNING *"
    )
    .bind(&now)
    .bind(&program_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(updated)
}

// Badge Commands (2 commands)

#[tauri::command]
pub async fn get_badges(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<Badge>, String> {
    let badges = sqlx::query_as::<_, Badge>(
        "SELECT * FROM badges WHERE program_id = ? ORDER BY earned_at DESC"
    )
    .bind(&program_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    Ok(badges)
}

#[tauri::command]
pub async fn check_and_award_badges(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<Badge>, String> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut new_badges = Vec::new();
    
    // Check various badge conditions and award if met
    // This is a simplified version - full implementation would check all 14 badge types
    
    // First Step badge
    let attempt_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM day_attempts da
         JOIN day_plans dp ON da.day_plan_id = dp.id
         WHERE dp.program_id = ? AND da.is_draft = 0"
    )
    .bind(&program_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    if attempt_count >= 1 {
        let existing: Option<String> = sqlx::query_scalar(
            "SELECT id FROM badges WHERE program_id = ? AND badge_type = 'first_step'"
        )
        .bind(&program_id)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;
        
        if existing.is_none() {
            let badge = sqlx::query_as::<_, Badge>(
                "INSERT INTO badges (id, program_id, badge_type, title, description, icon, earned_at, metadata)
                 VALUES (?, ?, 'first_step', 'First Step', 'Completed your first day', '🎯', ?, '{}')
                 RETURNING *"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&program_id)
            .bind(&now)
            .fetch_one(pool.inner())
            .await
            .map_err(|e: sqlx::Error| e.to_string())?;
            
            new_badges.push(badge);
        }
    }
    
    Ok(new_badges)
}

// Skill Score Commands (2 commands)

#[tauri::command]
pub async fn get_skill_radar(
    pool: State<'_, SqlitePool>,
    program_id: String,
) -> Result<Vec<SkillRadarData>, String> {
    let scores = sqlx::query_as::<_, SkillScore>(
        "SELECT * FROM skill_scores WHERE program_id = ? ORDER BY domain"
    )
    .bind(&program_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    let radar_data: Vec<SkillRadarData> = scores
        .into_iter()
        .map(|s| SkillRadarData {
            domain: s.domain,
            score: s.score,
            max_score: 100,
        })
        .collect();
    
    Ok(radar_data)
}

#[tauri::command]
pub async fn update_skill_scores(
    pool: State<'_, SqlitePool>,
    program_id: String,
    day_attempt_id: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    
    // Get attempt score and associated concept tags
    let attempt = sqlx::query_as::<_, DayAttempt>(
        "SELECT * FROM day_attempts WHERE id = ?"
    )
    .bind(&day_attempt_id)
    .fetch_one(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    let tags = sqlx::query_as::<_, ConceptTag>(
        "SELECT ct.* FROM concept_tags ct
         JOIN day_plan_tags dpt ON ct.id = dpt.concept_tag_id
         WHERE dpt.day_plan_id = ?"
    )
    .bind(&attempt.day_plan_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e: sqlx::Error| e.to_string())?;
    
    // Update skill score for each domain
    for tag in tags {
        sqlx::query(
            "INSERT INTO skill_scores (id, program_id, domain, score, total_attempts, last_updated, created_at)
             VALUES (?, ?, ?, ?, 1, ?, ?)
             ON CONFLICT(program_id, domain) DO UPDATE SET
                score = ((skill_scores.score * skill_scores.total_attempts) + ?) / (skill_scores.total_attempts + 1),
                total_attempts = skill_scores.total_attempts + 1,
                last_updated = ?"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&program_id)
        .bind(&tag.domain)
        .bind(attempt.total_score)
        .bind(&now)
        .bind(&now)
        .bind(attempt.total_score)
        .bind(&now)
        .execute(pool.inner())
        .await
        .map_err(|e: sqlx::Error| e.to_string())?;
    }
    
    Ok(())
}

// Helper functions for SM-2 algorithm

fn map_score_to_quality(score: i32) -> i32 {
    match score {
        0..=49 => 0,
        50..=59 => 1,
        60..=69 => 2,
        70..=79 => 3,
        80..=89 => 4,
        _ => 5,
    }
}

fn calculate_easiness_factor(current_ef: f64, quality: i32) -> f64 {
    let new_ef = current_ef + (0.1 - (5.0 - quality as f64) * (0.08 + (5.0 - quality as f64) * 0.02));
    new_ef.max(1.3)
}

fn calculate_interval(current_interval: i32, repetition_count: i32, quality: i32) -> i32 {
    if quality < 3 {
        1
    } else if repetition_count == 0 {
        1
    } else if repetition_count == 1 {
        6
    } else {
        (current_interval as f64 * 2.5) as i32
    }
}
