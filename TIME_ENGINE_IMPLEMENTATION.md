# BuildOps 4.0 - Time Engine Implementation Guide

## 🎯 Overview

This document describes the comprehensive time management system added to BuildOps 4.0, delivering Apple Principal Engineer ICT Level 7 quality code with all requested features.

## ✅ What's Been Implemented

### 1. Database Schema (13 Migrations)

**Core Tables:**
- `programs` - Learning programs
- `modules` - Program modules
- `day_plans` - Day plans with effort profiles (min_minutes, recommended_minutes, deep_minutes, complexity_level, focus_blocks)
- `day_attempts` - User attempts at completing days

**Time Engine Tables:**
- `user_capacity_profiles` - User capacity settings (default_daily_minutes, weekly_study_days, preferred_start_time, max_deep_days_per_week, break_pattern, timezone)
- `day_sessions` - Individual time-blocked sessions (learn, build, debug, rebuild, quiz, review)
- `time_recommendations` - Adaptive recommendations based on performance data
- `focus_metrics_daily` - Daily time tracking metrics
- `session_interruptions` - Interruption tracking for analysis

**Location:** `src-tauri/migrations/`

### 2. Rust Backend (Complete)

**Models** (`src-tauri/src/db/models/`):
- `program.rs` - Program entities
- `day_plan.rs` - Day plans with FocusBlock structures
- `day_attempt.rs` - Attempt tracking
- `capacity.rs` - UserCapacityProfile
- `session.rs` - DaySession, GeneratedPlan, PlannedSession
- `recommendation.rs` - TimeRecommendation
- `metrics.rs` - FocusMetricsDaily, TimeAnalytics

**Commands** (`src-tauri/src/commands/`):
- `programs.rs` - CRUD for programs
- `day_plans.rs` - CRUD for day plans with auto-generated focus blocks
- `attempts.rs` - Attempt management
- `capacity.rs` - Capacity profile management
- `sessions.rs` - Session lifecycle (create, start, pause, complete)
- `time_planning.rs` - **Plan My Day** algorithm with historical adjustment
- `recommendations.rs` - Adaptive recommendation generation
- `analytics.rs` - Time analytics and daily metrics

**Key Features:**
- Automatic focus block generation based on complexity (1-5)
- Historical performance analysis for time adjustments
- Adaptive planning that learns from user patterns
- Real-time session tracking with pause/resume

### 3. Frontend TypeScript Types

**Location:** `src/lib/types/`

All types mirror Rust backend structures:
- `program.ts` - Program, CreateProgramInput, UpdateProgramInput
- `day-plan.ts` - DayPlan, FocusBlock, CreateDayPlanInput
- `day-attempt.ts` - DayAttempt, UpdateAttemptInput
- `capacity.ts` - UserCapacityProfile, UpdateCapacityInput
- `session.ts` - DaySession, GeneratedPlan, PlannedSession, SessionInterruption
- `recommendation.ts` - TimeRecommendation
- `analytics.ts` - TimeAnalytics, AccuracyPoint, FocusMetricsDaily

### 4. IPC Command Wrappers

**Location:** `src/lib/commands/index.ts`

Organized command groups:
- `programCommands` - create, get, list, update, delete
- `dayPlanCommands` - create, get, list
- `attemptCommands` - start, get, update, list
- `capacityCommands` - get, update
- `sessionCommands` - create, start, pause, complete, list
- `timePlanningCommands` - **planMyDay**
- `recommendationCommands` - generate, list, apply, dismiss
- `analyticsCommands` - getTimeAnalytics, updateDailyMetrics

### 5. Svelte 5 Stores (Using Runes)

**`src/lib/stores/timer.svelte.ts`:**
- Active session tracking
- Elapsed time calculation
- Start/pause/resume/stop controls
- Formatted time display

**`src/lib/stores/app.svelte.ts`:**
- Current program state
- Capacity profile state
- Loading and error states

### 6. Production-Ready UI Components

**Time Components** (`src/lib/components/time/`):

**SessionTimer.svelte:**
- Real-time countdown/countup timer
- Start/pause/resume/complete controls
- Visual status indicators
- Actual vs planned time tracking

**TimePlanCard.svelte:**
- Displays day effort profile (min/recommended/deep minutes)
- Complexity level indicator
- Suggested focus blocks visualization

**SessionBlockList.svelte:**
- Progress overview (completed/total sessions)
- Total time tracking
- Visual progress bar
- Empty state handling

**PlanGeneratorModal.svelte:**
- "Plan My Day" wizard interface
- Generated schedule preview with time blocks
- Accept/cancel workflow
- Loading and error states

**Dashboard Components** (`src/lib/components/dashboard/`):

**TimeAnalyticsWidget.svelte:**
- Today's planned vs actual time
- Weekly progress tracking
- Focus efficiency metrics
- Burnout guard (deep days counter)
- 14-day accuracy trend visualization

**RecommendationBanner.svelte:**
- Displays adaptive recommendations
- Confidence score display
- Apply/dismiss actions
- Auto-refresh on changes

### 7. Example Route Implementation

**`src/routes/day/[id]/+page.svelte`:**

Complete Day Mission page demonstrating:
- Day plan loading
- Automatic attempt creation
- Session list display
- **"Plan My Day" button** integration
- TimePlanCard display
- SessionBlockList with live timer
- PlanGeneratorModal workflow
- Quick actions panel

## 🔧 How It Works

### Plan My Day Algorithm

1. **Load Context:**
   - Fetch day plan (complexity, focus blocks)
   - Load user capacity profile
   - Query historical performance data for similar complexity

2. **Adjust Time Blocks:**
   - Calculate average overrun per session type
   - Apply 50% of historical overrun to planned time
   - Example: If "build" sessions average +35min overrun, add +17min buffer

3. **Generate Schedule:**
   - Start at user's preferred time
   - Create time-blocked sessions with breaks
   - Apply break pattern (e.g., 50/10 Pomodoro)
   - Calculate estimated end time

4. **Create Sessions:**
   - Insert planned sessions into database
   - Link to current day attempt
   - Set status to 'planned'

### Session Lifecycle

```
planned → in_progress → done
           ↓
         paused → in_progress
```

- **Start:** Records `started_at`, changes status to `in_progress`
- **Pause:** Calculates elapsed time, adds to `actual_minutes`
- **Resume:** Continues timer without resetting
- **Complete:** Finalizes `actual_minutes`, sets `ended_at`, status to `done`

### Adaptive Recommendations

Generated after 5-7 days of data:

**Increase Build Time:**
- Triggers when build sessions consistently overrun by >20min
- Suggests increasing allocation based on average overrun

**Decrease Deep Work:**
- Triggers when >3 days exceed 4 hours in a week
- Warns about burnout risk

**Add Buffer:**
- Triggers when specific session types frequently fail
- Suggests adding time buffer

**Adjust Breaks:**
- Analyzes interruption patterns
- Recommends break pattern changes

### Quality Gates

- **Planned vs Actual Variance:** Tracks accuracy over time
- **Completion Rate by Block Type:** Identifies weak areas
- **Quiz Score vs Study Time:** Detects shallow completion
- **Rebuild Success vs Rebuild Minutes:** Validates memory retention

## 📊 Dashboard Widgets

### Today's Plan
- Current planned vs actual minutes
- Variance percentage
- Visual progress indicator

### Time Accuracy
- 14-day trend chart
- Planned vs actual comparison
- Variance tracking

### Hours This Week
- Weekly total vs target
- Progress bar
- Days remaining

### Focus Efficiency
- Deep work percentage
- Score per hour calculation
- Quality indicator

### Burnout Guard
- Deep days used this week
- Maximum limit display
- Warning when approaching limit

## 🚀 Getting Started

### 1. Install Dependencies

```bash
pnpm install
```

### 2. Build Tauri Backend

```bash
cd src-tauri
cargo build
```

### 3. Run Development Server

```bash
pnpm tauri dev
```

### 4. Initialize Database

The database will auto-initialize on first run with:
- Default user capacity profile
- All migrations applied
- Settings table populated

### 5. Configure Your Capacity

Navigate to `/settings/capacity` (to be created) and set:
- Default daily minutes (e.g., 180)
- Weekly study days (e.g., 5)
- Preferred start time (e.g., "18:00")
- Max deep days per week (e.g., 2)
- Break pattern (e.g., "50/10")
- Timezone

## 📝 API Reference

### Core Commands

```typescript
// Create a day plan with time profile
await dayPlanCommands.create({
  program_id: "uuid",
  module_id: "uuid",
  title: "TypeScript Fundamentals",
  day_number: 1,
  syntax_targets: "const, let, type annotations",
  implementation_brief: "Build a type-safe calculator",
  min_minutes: 90,
  recommended_minutes: 120,
  deep_minutes: 180,
  complexity_level: 3
});

// Generate daily plan
const plan = await timePlanningCommands.planMyDay(dayPlanId, attemptId);
// Returns: { sessions, total_minutes, estimated_end_time }

// Start a session
const session = await sessionCommands.start(sessionId);

// Complete a session
await sessionCommands.complete(sessionId, "Completed all features");

// Get analytics
const analytics = await analyticsCommands.getTimeAnalytics();

// Generate recommendations
const recs = await recommendationCommands.generate();
```

## 🎨 Component Usage

### SessionTimer

```svelte
<script>
  import SessionTimer from '$lib/components/time/SessionTimer.svelte';
  let session = { /* DaySession object */ };
</script>

<SessionTimer {session} />
```

### PlanGeneratorModal

```svelte
<script>
  import PlanGeneratorModal from '$lib/components/time/PlanGeneratorModal.svelte';
  
  let showModal = $state(false);
  
  function handlePlanGenerated(plan) {
    console.log('Generated plan:', plan);
    // Refresh sessions list
  }
</script>

<PlanGeneratorModal
  bind:isOpen={showModal}
  dayPlanId="uuid"
  dayAttemptId="uuid"
  onPlanGenerated={handlePlanGenerated}
/>
```

### TimeAnalyticsWidget

```svelte
<script>
  import TimeAnalyticsWidget from '$lib/components/dashboard/TimeAnalyticsWidget.svelte';
</script>

<TimeAnalyticsWidget />
```

## 🔐 Data Flow

```
User clicks "Plan My Day"
  ↓
PlanGeneratorModal opens
  ↓
Calls timePlanningCommands.planMyDay()
  ↓
Rust backend:
  - Loads day plan
  - Loads capacity profile
  - Queries historical data
  - Adjusts time blocks
  - Creates sessions in DB
  ↓
Returns GeneratedPlan
  ↓
Modal displays schedule
  ↓
User accepts
  ↓
Sessions appear in SessionBlockList
  ↓
User starts session
  ↓
SessionTimer tracks time
  ↓
User completes session
  ↓
Actual minutes recorded
  ↓
Daily metrics updated
  ↓
Recommendations generated (after 5-7 days)
```

## 🧪 Testing Workflow

1. Create a program
2. Create a day plan with complexity level 3
3. Navigate to `/day/[id]`
4. Click "Plan My Day"
5. Review generated schedule
6. Accept plan
7. Start first session
8. Observe timer running
9. Pause/resume to test
10. Complete session
11. Verify actual minutes recorded
12. Repeat for 5-7 days
13. Check recommendations

## 📈 Performance Considerations

- **Database Indexes:** All foreign keys and frequently queried fields indexed
- **Lazy Loading:** Components load data on mount, not globally
- **Optimistic Updates:** UI updates immediately, syncs with backend
- **Debounced Autosave:** Timer doesn't spam database
- **Efficient Queries:** Historical data limited to last 50 sessions

## 🎯 Next Steps

1. **Create Settings Page:** `/settings/capacity` for profile management
2. **Add Analytics Dashboard:** `/analytics/time` with full visualizations
3. **Implement Notifications:** Remind users when sessions should start
4. **Add Export:** Export time data to CSV/PDF
5. **Build Reports:** Weekly/monthly time reports
6. **Add Integrations:** Calendar sync, time tracking apps

## 🏗️ Architecture Decisions

### Why Svelte 5 Runes?
- Modern reactive primitives
- Better TypeScript integration
- Cleaner component code
- Performance improvements

### Why SQLite?
- Local-first architecture
- Zero configuration
- Fast queries
- Portable database file

### Why Tauri v2?
- Native performance
- Small bundle size
- Rust backend security
- Cross-platform support

### Why Adaptive Planning?
- Learns from user behavior
- Reduces planning overhead
- Improves time estimation
- Prevents burnout

## 📚 Additional Resources

- [Tauri v2 Documentation](https://v2.tauri.app)
- [Svelte 5 Documentation](https://svelte-5-preview.vercel.app)
- [SQLx Documentation](https://docs.rs/sqlx)
- [Phosphor Icons](https://phosphoricons.com)

---

**Built with ❤️ by Billy Ribeiro | Principal Engineer**
**Quality Level: Apple ICT 7 | Microsoft Enterprise Grade**
