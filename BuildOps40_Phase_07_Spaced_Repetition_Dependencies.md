# BuildOps 40 — Phase 7: Spaced Repetition & Dependency Intelligence

## What This Phase Builds

Two systems that enforce long-term retention and proper learning sequencing. Spaced repetition uses the SM-2 algorithm to resurface concepts at scientifically optimal intervals. The dependency graph prevents skipping ahead when prerequisites aren't met.

---

## Dependencies on Phase 6

Phase 6 must be complete. Scoring, streaks, and badges must be operational.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/services/spaced_repetition.rs` — SM-2 algorithm implementation, review scheduling, due review queries
2. `src-tauri/src/services/dependency.rs` — dependency graph resolution, topological sort for circular detection, unmet dependency queries
3. `src-tauri/src/commands/spaced_repetition.rs` — 3 commands: get_due_reviews, record_review, get_review_schedule
4. Update `src-tauri/src/commands/attempts.rs` — after submit, create/update spaced repetition entries
5. Update `src-tauri/src/commands/days.rs` — `check_dependencies` now queries actual attempt scores, `add_dependency` validates no circular chains
6. `src-tauri/src/commands/analytics.rs` — add `get_forgetting_curve_alerts`
7. Update `src-tauri/src/lib.rs` — register new commands

### Frontend Layer
8. `src/lib/commands/spaced_repetition.ts` — typed wrappers
9. `src/lib/utils/spaced-repetition.ts` — client-side quality mapping helper
10. `src/lib/components/dashboard/UpcomingDays.svelte` — update to include spaced repetition due reviews
11. `src/lib/components/analytics/ForgettingCurveAlert.svelte` — alert component for concepts at risk of being forgotten
12. `src/lib/components/program/DependencyGraph.svelte` — visual dependency chain display (simple list view, not a full graph visualization — that's polish)
13. Update dashboard page to show due review count and review prompt
14. Update day mission page dependency checks to use real scored data

---

## SM-2 Algorithm Implementation

```rust
pub struct SmTwoResult {
    pub easiness_factor: f64,
    pub interval_days: i32,
    pub repetitions: i32,
    pub next_review_date: String,
}

pub fn calculate_sm2(
    quality: i32,           // 0-5
    current_ef: f64,        // current easiness factor
    current_interval: i32,  // current interval in days
    current_reps: i32,      // current repetition count
) -> SmTwoResult {
    let quality = quality.clamp(0, 5);
    
    let (new_reps, new_interval) = if quality < 3 {
        // Failed review — reset
        (0, 1)
    } else {
        let reps = current_reps + 1;
        let interval = match reps {
            1 => 1,
            2 => 6,
            _ => (current_interval as f64 * current_ef).round() as i32,
        };
        (reps, interval.max(1))
    };
    
    // Update easiness factor
    let q = quality as f64;
    let new_ef = (current_ef + (0.1 - (5.0 - q) * (0.08 + (5.0 - q) * 0.02))).max(1.3);
    
    let next_review = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(new_interval as i64))
        .unwrap()
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();
    
    SmTwoResult {
        easiness_factor: new_ef,
        interval_days: new_interval,
        repetitions: new_reps,
        next_review_date: next_review,
    }
}
```

### Quality Mapping from App Scores

| Total Score | SM-2 Quality | Meaning |
|-------------|-------------|---------|
| 0–49 | 0 | Complete blackout |
| 50–59 | 1 | Incorrect but remembered something |
| 60–69 | 2 | Incorrect but easy to recall correct |
| 70–79 | 3 | Correct with serious difficulty |
| 80–89 | 4 | Correct with hesitation |
| 90–100 | 5 | Perfect recall |

### Spaced Repetition Entry Creation

When `submit_attempt` completes, upsert a `spaced_repetition` entry for the day_plan:
- If entry doesn't exist: create with defaults (EF=2.5, interval=1, reps=0)
- Map total_score to SM-2 quality (0-5)
- Run SM-2 calculation
- Update entry with new values

### Due Reviews Query

`get_due_reviews` returns all day plans where `next_review_date <= today` for a given program. Ordered by most overdue first.

### Review Schedule

`get_review_schedule` returns upcoming reviews for the next N days as a list of (date, count, day_plan_summaries). Useful for the dashboard "upcoming reviews" widget.

---

## Dependency Intelligence

### Circular Dependency Detection

When `add_dependency(A depends on B)` is called:
1. Build the full dependency chain starting from B
2. If A appears anywhere in B's dependency chain, reject with error "Circular dependency detected: {chain}"
3. Use iterative depth-first traversal (not recursive) to avoid stack overflow on deep chains

```rust
pub async fn check_circular(
    pool: &SqlitePool,
    day_plan_id: &str,
    depends_on_id: &str,
) -> Result<bool, AppError> {
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
        
        let deps = sqlx::query_scalar::<_, String>(
            "SELECT depends_on_day_plan_id FROM day_dependencies WHERE day_plan_id = ?"
        )
        .bind(&current)
        .fetch_all(pool)
        .await?;
        
        stack.extend(deps);
    }
    
    Ok(false)
}
```

### Dependency Status Check

`check_dependencies` for a day plan returns enriched status for each dependency:

```rust
pub struct DependencyStatus {
    pub dependency_id: String,
    pub depends_on_day_plan_id: String,
    pub depends_on_title: String,
    pub depends_on_day_number: i32,
    pub dependency_type: String,      // prerequisite, recommended, related
    pub minimum_score: i32,
    pub is_met: bool,
    pub best_score: Option<i32>,       // Best attempt score for depended day
    pub best_attempt_status: Option<String>,
}
```

For `prerequisite` type: `is_met = best_score >= minimum_score`
For `recommended` type: always `is_met = true` (shown as suggestion, not enforcer)
For `related` type: always `is_met = true` (informational only)

### Forgetting Curve Alerts

`get_forgetting_curve_alerts` returns concept domains where:
- The user hasn't touched a day plan tagged with that domain in > 14 days
- AND their best score for that domain was below mastery (< 95)
- Ordered by most at-risk first (longest since last review + lowest score)

---

## Verification Checklist

1. **Spaced repetition entry created:** Submit an attempt. Check `spaced_repetition` table — entry exists for that day_plan with calculated next_review_date.

2. **Due reviews appear:** Manually set a `next_review_date` to yesterday in the DB. Call `get_due_reviews`. Verify the day plan appears.

3. **Record review:** Call `record_review` with quality 4. Verify interval increased and next_review_date pushed forward.

4. **Low quality review:** Call `record_review` with quality 1. Verify interval reset to 1 day and repetitions reset to 0.

5. **Dashboard shows due count:** Verify dashboard displays "3 reviews due today" (or whatever the count is).

6. **Dependency blocks attempt:** Day 2 depends on Day 1 (min score 70). Day 1 has no passing attempt. Verify Day 2 mission page shows "Start Attempt" as disabled with dependency warning.

7. **Dependency met:** Submit Day 1 with score 75. Navigate to Day 2. Verify dependencies show as met and "Start Attempt" is enabled.

8. **Circular dependency rejected:** Day 2 depends on Day 1. Try to add Day 1 depends on Day 2. Verify error returned.

9. **Forgetting curve alerts:** Create a day plan tagged with domain "CSS". Submit with score 80. Set the submission date to 20 days ago in DB. Call `get_forgetting_curve_alerts`. Verify CSS domain appears as at-risk.

10. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- SM-2 spaced repetition algorithm fully implemented
- Review scheduling with due date tracking
- Forgetting curve alerts for at-risk concepts
- Circular dependency prevention
- Dependency status checking with real score data
- Dashboard integration for due reviews

## Next Phase

Phase 8 builds the Analytics Dashboard — all charts, visualizations, and the weekly review system.
