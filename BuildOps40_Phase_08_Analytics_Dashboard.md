# BuildOps 40 — Phase 8: Analytics Dashboard & Weekly Reviews

## What This Phase Builds

All charts, visualizations, and the complete analytics experience. The dashboard becomes a real command center with live data. Weekly reviews auto-generate from attempt data. The skill radar, burndown chart, concept heatmap, and all other visualizations come to life.

---

## Dependencies on Phase 7

Phase 7 must be complete. Scoring, streaks, badges, spaced repetition, and dependencies must all be operational.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/services/review.rs` — weekly review auto-generation logic
2. `src-tauri/src/commands/analytics.rs` — complete with all 13 analytics commands:
   - `get_dashboard_data` — aggregated dashboard payload
   - `get_skill_radar` — skill scores per domain
   - `get_weekly_review` — fetch specific weekly review
   - `generate_weekly_review` — auto-generate current week's review
   - `get_burndown_data` — remaining days vs pace
   - `get_concept_heatmap` — concept × score grid
   - `get_score_history` — score timeline data points
   - `get_time_analytics` — planned vs actual time data
   - `get_bug_analytics` — category/severity distributions
   - `get_streak_data` — (already exists from Phase 6)
   - `use_streak_freeze` — (already exists from Phase 6)
   - `get_badges` — (already exists from Phase 6)
   - `get_forgetting_curve_alerts` — (already exists from Phase 7)
3. Update `src-tauri/src/lib.rs` — register new commands

### Frontend Layer
4. `src/lib/commands/analytics.ts` — typed wrappers for all analytics commands
5. `src/lib/components/dashboard/TodayCard.svelte` — today's mission with quick-start
6. `src/lib/components/dashboard/ProgressRingChart.svelte` — animated SVG ring showing completion %
7. `src/lib/components/dashboard/QualityScoreTrend.svelte` — sparkline of last 7 day scores
8. `src/lib/components/dashboard/BlockedAlerts.svelte` — list of blocked days requiring replay
9. `src/lib/components/dashboard/RecentActivity.svelte` — last 5 actions feed
10. `src/lib/components/analytics/SkillRadar.svelte` — ECharts radar chart
11. `src/lib/components/analytics/ScoreTrendLine.svelte` — ECharts line chart of score history
12. `src/lib/components/analytics/ConceptHeatmap.svelte` — ECharts heatmap grid
13. `src/lib/components/analytics/BurndownChart.svelte` — ECharts line chart with ideal + actual pace
14. `src/lib/components/analytics/TimeTracker.svelte` — ECharts stacked bar of planned vs actual time
15. `src/lib/components/analytics/WeeklyReviewCard.svelte` — weekly review display + edit
16. Update `src/routes/+page.svelte` — full dashboard with all widgets (replace placeholder)
17. `src/routes/analytics/+page.svelte` — analytics hub with all charts (replace placeholder)
18. `src/routes/analytics/+page.ts`
19. `src/routes/analytics/skills/+page.svelte` — skill radar deep dive
20. `src/routes/analytics/weekly/+page.svelte` — weekly reviews list
21. `src/routes/analytics/weekly/[weekId]/+page.svelte` — single weekly review detail
22. `src/routes/analytics/burndown/+page.svelte` — burndown chart full page

---

## Dashboard Data Shape

`get_dashboard_data` returns a single payload to minimize IPC calls:

```rust
pub struct DashboardData {
    pub today_day: Option<DayPlanSummary>,       // Next incomplete day plan
    pub progress_completed: i32,                  // Days with passing attempt
    pub progress_total: i32,                      // Total day plans in program
    pub progress_percent: f64,
    pub current_streak: i32,
    pub longest_streak: i32,
    pub streak_freezes_available: i32,
    pub quality_trend: Vec<ScorePoint>,           // Last 7 submitted scores
    pub blocked_days: Vec<BlockedDayInfo>,         // Days with blocked status
    pub due_reviews_count: i32,                    // Spaced repetition due today
    pub recent_activity: Vec<ActivityEntry>,       // Last 5 actions
    pub upcoming_days: Vec<DayPlanSummary>,        // Next 3 incomplete days
    pub new_badges: Vec<Badge>,                    // Badges earned in last 24 hours
}
```

The Rust command runs ~8 queries in a single function and assembles the payload. All queries are optimized with indices from the migrations.

---

## Weekly Review Auto-Generation

`generate_weekly_review` calculates the current week (Monday-Sunday) and:

1. Queries all submitted attempts within the date range for this program
2. Calculates: days_completed, days_blocked, average_score, best_score, worst_score
3. Sums total_time_minutes from time_logs within the range
4. Queries concept tags for high-scoring days (≥85) → strong_concepts
5. Queries concept tags for low-scoring days (<70) → weak_concepts
6. Identifies blocked days that should be replayed → replay_recommendations
7. Creates or updates the weekly_reviews record
8. Returns the review

The user can also manually edit the `summary` and `goals_next_week` fields.

---

## Chart Specifications

### Skill Radar (ECharts)

- Type: radar chart
- Axes: one per domain found in skill_scores table (dynamically generated)
- Values: domain scores (0-100)
- Style: filled area with semi-transparent accent color, border line
- Show value labels at each axis point

### Score Trend Line (ECharts)

- Type: line chart with area fill
- X axis: date of submission
- Y axis: total score (0-100)
- Data points: every submitted attempt, ordered by date
- Horizontal reference lines at 70 (blocked threshold) and 95 (mastery threshold)
- Color: points below 70 are red, 70-94 are green, 95+ are purple
- Optional: 7-day moving average overlay

### Concept Heatmap (ECharts)

- Type: heatmap
- X axis: concept tag names
- Y axis: day plan titles (or day numbers)
- Cell value: score for that day's attempt
- Color scale: red (0) → yellow (70) → green (100)
- Only shows concepts that have been tagged to at least one day

### Burndown Chart (ECharts)

- Type: dual line chart
- X axis: date range from program start to projected end
- Line 1 (dashed gray): ideal pace — linear from total_days to 0
- Line 2 (solid accent): actual remaining — total_days minus completed days at each date
- Area between lines colored: green if ahead, red if behind

### Time Distribution (ECharts)

- Type: grouped bar chart
- X axis: day numbers
- Bar 1: estimated_minutes (from day plan)
- Bar 2: actual_minutes (from attempt)
- Color: green when actual ≤ estimated, amber when actual > estimated

### Bug Category Distribution (ECharts)

- Type: donut chart
- Slices: bug log categories
- Inner label: total bug count
- Click slice to filter bug list

---

## Dashboard Layout

```
┌─────────────────────────────────────────────────┐
│  Today's Mission Card (wide)                    │
│  [Day 15 — GSAP ScrollTrigger] [Start] [85pts] │
├──────────┬──────────┬──────────┬────────────────┤
│ Progress │ Streak   │ Quality  │ Due Reviews    │
│ Ring 62% │ 🔥 12    │ Trend ↑  │ 3 concepts     │
├──────────┴──────────┴──────────┴────────────────┤
│  Blocked Alerts (if any)                        │
│  ⚠ Day 11 (score 58) blocks Day 15, Day 16     │
├──────────────────────┬──────────────────────────┤
│  Recent Activity     │  Upcoming Days           │
│  - Submitted Day 14  │  Day 15: GSAP Scroll     │
│  - Earned badge      │  Day 16: GSAP Timeline   │
│  - 3 bugs logged     │  Day 17: Advanced Forms  │
└──────────────────────┴──────────────────────────┘
```

Responsive: on smaller windows, cards stack vertically.

---

## Analytics Hub Layout (`/analytics`)

Full-page analytics with tab or section navigation:

1. **Overview** — Skill Radar (large) + Score Trend (large) side by side
2. **Concepts** — Concept Heatmap (full width)
3. **Time** — Time Distribution chart + summary stats
4. **Bugs** — Bug category donut + severity breakdown + recurring patterns list
5. **Progress** — Burndown chart + completion timeline

Each section is collapsible. Charts are interactive (hover for tooltips, click for drill-down where applicable).

---

## Verification Checklist

1. **Dashboard renders:** With at least 5 submitted attempts in a program, load dashboard. Verify: TodayCard shows next incomplete day, progress ring shows correct %, streak shows, quality trend sparkline has 5+ points, blocked alerts show if any blocked days exist.

2. **Skill radar:** Navigate to analytics. Verify radar chart has axes for each domain with data points.

3. **Score trend:** Verify line chart plots all submitted attempt scores chronologically.

4. **Concept heatmap:** Verify grid shows concept × day matrix with color-coded scores.

5. **Burndown chart:** Verify ideal line starts at total_days and goes to 0. Verify actual line follows real completion data.

6. **Time analytics:** Verify bars show planned vs actual for each attempted day.

7. **Bug analytics:** Verify donut chart shows bug categories.

8. **Weekly review generated:** Click "Generate Review" for current week. Verify review populates with correct aggregated data.

9. **Edit weekly review:** Add a manual summary and goals. Save. Verify persistence.

10. **Empty states:** View analytics for a program with zero attempts. Verify charts show meaningful empty states (not broken/blank).

11. **Chart responsiveness:** Resize window. Verify charts resize properly.

12. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete dashboard with all widgets and live data
- All 6 chart types operational (radar, line, heatmap, burndown, bar, donut)
- Weekly review auto-generation + manual editing
- Analytics hub with full visualization suite
- Bug pattern analysis visible

## Next Phase

Phase 9 builds Search and the Command Palette — full-text search across all content and keyboard-driven navigation.
