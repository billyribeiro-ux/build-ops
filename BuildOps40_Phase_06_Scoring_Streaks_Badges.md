# BuildOps 40 — Phase 6: Scoring Engine, Streaks & Badges

## What This Phase Builds

The intelligence layer. When a user submits an attempt, the scoring engine evaluates quality gates, the streak tracker updates, the badge evaluator checks for newly earned achievements, and skill scores recalculate. This phase transforms raw scores into meaningful progression data.

---

## Dependencies on Phase 5

Phase 5 must be complete. Day attempts must be submittable with scores.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/services/mod.rs` — service module declarations
2. `src-tauri/src/services/scoring.rs` — score calculation, quality gate evaluation, skill score updates
3. `src-tauri/src/services/streak.rs` — streak tracking, freeze mechanic, streak reset logic
4. `src-tauri/src/services/badge.rs` — badge condition evaluation, badge creation
5. `src-tauri/src/db/models/streak.rs` — Streak struct + queries
6. `src-tauri/src/db/models/badge.rs` — Badge struct + queries
7. `src-tauri/src/db/models/skill_score.rs` — SkillScore struct + queries
8. Update `src-tauri/src/commands/attempts.rs` — `submit_attempt` now calls scoring service, streak service, and badge service after saving scores
9. `src-tauri/src/commands/analytics.rs` — initial analytics commands: get_streak_data, use_streak_freeze, get_badges
10. Update `src-tauri/src/lib.rs` — register new commands

### Frontend Layer
11. `src/lib/types/analytics.ts` — StreakData, Badge, SkillScore types
12. `src/lib/commands/analytics.ts` — typed wrappers
13. `src/lib/components/dashboard/StreakDisplay.svelte` — current streak + longest streak + freeze count
14. Update the submit flow in the working screen to show post-submission summary: score breakdown, quality gate result, streak update, any new badges earned

---

## Scoring Service (`services/scoring.rs`)

Called by `submit_attempt` after scores are saved to the attempt.

### Quality Gate Logic

```rust
pub fn evaluate_quality_gate(total_score: i32, memory_rebuild_passed: bool) -> AttemptStatus {
    if !memory_rebuild_passed {
        // Memory rebuild failed — blocked regardless of score
        // (only applies if memory_rebuild_completed is true)
        return AttemptStatus::Blocked;
    }
    
    match total_score {
        0..=69 => AttemptStatus::Blocked,
        70..=94 => AttemptStatus::Passed,
        95..=100 => AttemptStatus::Mastery,
        _ => AttemptStatus::Blocked, // Invalid score
    }
}
```

Note: If `memory_rebuild_completed` is false (user never attempted memory rebuild), the memory rebuild check is skipped. It only blocks if the user attempted and failed.

### Skill Score Update

After submission, update the skill scores for the program based on the day's concept tags:

1. Get all concept tags for the submitted day plan
2. For each tag's domain, recalculate the domain score:
   - Query all submitted attempts for day plans with tags in that domain within this program
   - Domain score = weighted average of those attempts' total scores
   - Weight recent attempts more heavily: `weight = 1.0 + (0.1 * recency_rank)` where most recent = highest rank
3. Upsert into `skill_scores` table

---

## Streak Service (`services/streak.rs`)

Called by `submit_attempt` after scoring.

### Streak Logic

```
On attempt submission:
  1. Get the streak record for this program (create if doesn't exist)
  2. Get today's date (local timezone)
  3. If last_active_date == today: no streak change (already active today)
  4. If last_active_date == yesterday: increment current_streak
  5. If last_active_date is older:
     a. Check if streak_freezes_available > 0 AND the gap is exactly 1 missed day
        - If yes: use a freeze, decrement streak_freezes_available, increment streak_freezes_used, increment current_streak
        - If no: reset current_streak to 1
  6. Update last_active_date to today
  7. Update longest_streak if current_streak > longest_streak
  8. Check if freeze_reset_month != current month
     - If different month: reset streak_freezes_available to max (from settings), reset streak_freezes_used to 0, update freeze_reset_month
```

### Use Streak Freeze Command

Manual freeze usage (for when user knows they'll miss tomorrow):
- Decrements streak_freezes_available
- No effect on current_streak until the next day
- The automatic freeze logic above handles the actual streak preservation

---

## Badge Service (`services/badge.rs`)

Called by `submit_attempt` after scoring and streak updates. Also callable independently for retroactive badge evaluation.

### Badge Evaluation

For each badge type, check if the condition is met and the badge hasn't already been awarded:

```rust
pub async fn evaluate_badges(
    pool: &SqlitePool,
    program_id: &str,
    day_plan_id: &str,
    attempt: &DayAttempt,
    streak: &Streak,
) -> Result<Vec<Badge>, AppError> {
    let mut new_badges = Vec::new();
    
    // Check each badge condition
    // Only award if not already earned for this scope
    
    // first_day: any attempt submitted
    if !has_badge(pool, program_id, "first_day").await? {
        award_badge(&mut new_badges, pool, "first_day", "First Step", "Completed your first day", program_id, Some(day_plan_id)).await?;
    }
    
    // streak_7, streak_14, streak_30
    if streak.current_streak >= 7 && !has_badge(pool, program_id, "streak_7").await? { ... }
    if streak.current_streak >= 14 && !has_badge(pool, program_id, "streak_14").await? { ... }
    if streak.current_streak >= 30 && !has_badge(pool, program_id, "streak_30").await? { ... }
    
    // mastery_day: score 95+
    if attempt.total_score >= 95 && !has_badge(pool, program_id, "mastery_day").await? { ... }
    
    // ... and so on for all 14 badge types
    
    Ok(new_badges)
}
```

### Badge Definitions (complete list)

| Badge ID | Title | Icon | Condition Query |
|----------|-------|------|----------------|
| `first_day` | First Step | `ph:flag-bold` | Any submitted attempt exists |
| `streak_7` | Week Warrior | `ph:fire-bold` | current_streak >= 7 |
| `streak_14` | Two-Week Titan | `ph:fire-bold` | current_streak >= 14 |
| `streak_30` | Monthly Machine | `ph:fire-bold` | current_streak >= 30 |
| `perfect_week` | Perfect Week | `ph:star-bold` | 7 consecutive days all >= 85 |
| `zero_blockers_14` | Clean Sheet | `ph:shield-check-bold` | Last 14 submitted attempts all not blocked |
| `mastery_day` | Mastery Achieved | `ph:crown-bold` | Any attempt total_score >= 95 |
| `mastery_5` | Mastery Streak | `ph:crown-bold` | 5 attempts with total_score >= 95 |
| `bug_hunter_100` | Bug Hunter | `ph:bug-bold` | 100+ bug_log entries across program |
| `memory_master` | Memory Master | `ph:brain-bold` | 10 consecutive memory_rebuild_passed = true |
| `full_evidence` | Evidence King | `ph:files-bold` | 20 day attempts with 3+ artifacts each |
| `program_complete` | Program Complete | `ph:trophy-bold` | All day plans in program have at least one attempt >= 70 |
| `speed_demon` | Speed Demon | `ph:lightning-bold` | actual_minutes < estimated_minutes AND score >= 85 |
| `comeback` | Comeback | `ph:arrow-u-up-right-bold` | Day previously blocked, now has attempt >= 85 |

---

## Post-Submission Flow Update

When `submit_attempt` is called, the Rust command now does this in order:

1. Validate and save scores to day_attempt
2. Calculate total_score
3. Evaluate quality gate → set status
4. Stop active time log, calculate actual_minutes
5. Set submitted_at, is_draft = false
6. Call `scoring::update_skill_scores()` for this program
7. Call `streak::update_streak()` for this program
8. Call `badge::evaluate_badges()` for this program + attempt
9. Return: updated DayAttempt + newly earned badges + updated streak

The frontend submission result type becomes:

```typescript
interface SubmitResult {
  attempt: DayAttempt;
  newBadges: Badge[];
  streak: StreakData;
}
```

The frontend shows a post-submission modal/overlay:
- Score breakdown with visual bars
- Quality gate result (big colored text)
- Streak update: "Day 12! 🔥" 
- If new badges earned: celebration display with badge icons and titles
- "View Day" / "Go to Dashboard" / "Start Next Day" buttons

---

## Verification Checklist

1. **Score gates work:** Submit attempt with score 65 → blocked. Score 75 → passed. Score 96 → mastery.
2. **Memory rebuild override:** Complete memory rebuild as failed, submit score 90 → blocked.
3. **Streak increments:** Submit on consecutive days (or simulate by adjusting last_active_date in DB). Verify streak increments.
4. **Streak breaks:** Skip a day. Submit again. Verify streak resets to 1.
5. **Streak freeze:** Set streak_freezes_available to 2. Skip exactly 1 day. Submit. Verify freeze used and streak preserved.
6. **Badge: first_day:** Submit first attempt. Verify "First Step" badge awarded.
7. **Badge: mastery_day:** Submit with score 95+. Verify "Mastery Achieved" badge.
8. **Badge: comeback:** Block a day (score <70). Replay and score 85+. Verify "Comeback" badge.
9. **Skill scores update:** Create days tagged with different domains. Submit attempts. Verify skill_scores table has entries for each domain with reasonable values.
10. **Post-submission UI:** Verify the summary modal shows correct score, gate, streak, and badges.
11. **No duplicate badges:** Submit another mastery attempt. Verify "Mastery Achieved" is NOT awarded again.
12. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete scoring engine with quality gates
- Skill score calculation per domain from concept tags
- Streak tracking with freeze mechanic and monthly reset
- All 14 badge types evaluated on every submission
- Post-submission celebration UI with score/streak/badge display
- Intelligence layer is operational

## Next Phase

Phase 7 builds Spaced Repetition and Dependency Intelligence — the systems that fight the forgetting curve and enforce learning prerequisites.
