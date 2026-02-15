# BuildOps 40 — Phase 4: Day Attempts & Working Screen

## What This Phase Builds

This is the heart of the app — the screen where learning actually happens. The user starts an attempt, writes code in an embedded editor, completes checklist items, takes notes, and submits scores. Autosave runs every 5 seconds. A session timer tracks time spent. The memory rebuild mechanic enforces genuine retention.

---

## Dependencies on Phase 3

Phase 3 must be complete. Day plans with checklists and quiz questions must be fully authorable.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/db/models/day_attempt.rs` — DayAttempt, DayAttemptFull, DayAttemptSummary, AutosaveInput, ScoreCardInput structs + queries
2. `src-tauri/src/db/models/time_log.rs` — TimeLog struct + queries
3. `src-tauri/src/commands/attempts.rs` — 18 Tauri commands (see command list below)
4. Update `src-tauri/src/lib.rs` — register all 18 new commands

### Frontend Layer
5. `src/lib/types/attempt.ts` — DayAttempt, DayAttemptFull, ScoreCard, TimeLog types
6. `src/lib/commands/attempts.ts` — typed invoke wrappers for all 18 commands
7. `src/lib/components/editor/CodeEditor.svelte` — CodeMirror 6 wrapper component with language selection, theme support, and change events
8. `src/lib/components/mission/MissionHeader.svelte` — day number, title, module badge, status, timer display
9. `src/lib/components/mission/SyntaxTargets.svelte` — rendered markdown of syntax targets
10. `src/lib/components/mission/TaskChecklist.svelte` — interactive checklist with completion tracking
11. `src/lib/components/mission/ScoreCardForm.svelte` — 5 score inputs with sliders, live total calculation, quality gate indicators
12. `src/lib/components/mission/MemoryRebuildPanel.svelte` — countdown timer, pass/fail toggle, notes field
13. `src/lib/components/mission/SessionTimer.svelte` — running clock showing elapsed time
14. `src/lib/components/mission/DayStatusBadge.svelte` — colored badge showing attempt status
15. `src/lib/stores/timer.svelte.ts` — session timer state management with start/stop/reset
16. `src/routes/programs/[programId]/days/[dayId]/attempt/+page.svelte` — the main working screen
17. `src/routes/programs/[programId]/days/[dayId]/attempt/+page.ts` — working screen data loader
18. `src/routes/programs/[programId]/days/[dayId]/history/+page.svelte` — attempt history with score comparisons
19. `src/routes/programs/[programId]/days/[dayId]/history/+page.ts` — history data loader
20. Update `src/routes/programs/[programId]/days/[dayId]/+page.svelte` — wire "Start Attempt" button

---

## Attempt Commands (18)

### Core Attempt Lifecycle (6)
| Command | Description |
|---------|-------------|
| `start_attempt` | Creates new DayAttempt with incremented attempt_number, status 'in_progress'. Initializes attempt_checklist rows for all checklist items (all unchecked). Starts a time_log with session_type 'implementation'. Returns the full attempt. |
| `get_attempt` | Returns DayAttemptFull: attempt data + checklist state + exercise entries + bug logs + artifacts + quiz attempts + active time log. Single call, all joined data. |
| `get_current_attempt` | For a given day_plan_id, returns the latest attempt with status 'in_progress' if one exists, otherwise null. Used to resume work after closing/reopening the app. |
| `list_attempts` | Returns Vec<DayAttemptSummary> for a day plan: id, attempt_number, status, total_score, started_at, submitted_at, actual_minutes. |
| `autosave_attempt` | Updates exercise_notes, code_snapshot, reflection fields (what_broke, why_broke, how_fixed, refactor_tomorrow, daily_summary), and last_autosave timestamp. Does NOT change status. This is called every 5 seconds from the frontend. Must be fast — simple UPDATE with no joins. |
| `submit_attempt` | Takes ScoreCardInput (5 score values). Validates each against max (40/20/15/15/10). Sets status based on total: <70 = 'blocked', 70-94 = 'passed', 95+ = 'mastery'. If memory_rebuild_completed but not passed, overrides to 'blocked'. Stops active time log. Sets submitted_at and actual_minutes (calculated from time logs). Sets is_draft to false. Returns updated attempt. |

### Checklist (1)
| Command | Description |
|---------|-------------|
| `toggle_checklist_item` | Toggles is_completed for a specific attempt_checklist row. Sets completed_at when checking, clears when unchecking. Returns new boolean state. |

### Memory Rebuild (2)
| Command | Description |
|---------|-------------|
| `start_memory_rebuild` | Creates a time_log with session_type 'memory_rebuild'. Sets memory_rebuild_completed = true on the attempt. Returns the time log. |
| `complete_memory_rebuild` | Sets memory_rebuild_passed (true/false) and memory_rebuild_notes on the attempt. Stops the memory rebuild time log. |

### Time Logs (3)
| Command | Description |
|---------|-------------|
| `start_time_log` | Creates new time_log for the attempt with given session_type and started_at = now. |
| `stop_time_log` | Sets ended_at = now, calculates duration_seconds. |
| `get_active_time_log` | Returns the most recent time_log for an attempt where ended_at is null, if any. |

### Exercise Entries (4) — moved here from Phase 5 because the working screen needs them
| Command | Description |
|---------|-------------|
| `add_exercise_entry` | Creates an exercise entry (code, markdown, or mixed) linked to an attempt. |
| `update_exercise_entry` | Updates content, title, language. |
| `delete_exercise_entry` | Removes entry, resequences order. |
| `reorder_exercise_entries` | Resets order indices. |

### Quiz Execution (2) — the quiz UI is simple enough to include here
| Command | Description |
|---------|-------------|
| `submit_quiz_answer` | Creates/updates quiz_attempt row for the given attempt + question. For multiple_choice, auto-grades is_correct. For other types, stores answer for self-grading. Records time_taken_seconds. |
| `get_quiz_results` | Returns all quiz_attempts for an attempt with the question data joined. Includes total score, time spent, per-question results. |

---

## Working Screen Layout (`/programs/[programId]/days/[dayId]/attempt`)

**This is the most complex screen in the app.** It must handle real-time autosave, timer management, and multiple content tabs.

### Overall Structure

```
┌──────────────────────────────────────────────────────────────────┐
│ MissionHeader: Day 5 — CSS Grid Layout │ Module: CSS │ 01:23:45 │
├────────────────────────────┬─────────────────────────────────────┤
│                            │                                     │
│  Day Plan Reference        │  Workspace Tabs                     │
│  (left pane, scrollable)   │  [Exercise] [Checklist] [Bugs]      │
│                            │  [Evidence] [Quiz]                  │
│  - Syntax Targets          │                                     │
│  - Implementation Brief    │  ┌─────────────────────────────┐    │
│  - Files to Create         │  │                             │    │
│  - Success Criteria        │  │  Active Tab Content         │    │
│  - Stretch Challenge       │  │  (CodeMirror editor,        │    │
│  - Notes                   │  │   checklist, forms, etc.)   │    │
│                            │  │                             │    │
│                            │  └─────────────────────────────┘    │
│                            │                                     │
├────────────────────────────┴─────────────────────────────────────┤
│ Footer: [Autosave: 3s ago] [Timer: 01:23:45] [Score] [Submit]   │
└──────────────────────────────────────────────────────────────────┘
```

- Left and right panes are resizable with a drag handle
- Default split: 35% left, 65% right
- Left pane shows the day plan content as read-only rendered markdown
- Right pane has 5 tabs

### Exercise Tab (default active)

- List of exercise entry blocks, each with:
  - Title input
  - Language selector dropdown (javascript, typescript, html, css, svelte, rust, python, json, markdown)
  - CodeMirror 6 editor instance with selected language mode and one-dark theme
  - Delete button
- "Add Exercise Block" button at bottom (creates new entry via `add_exercise_entry`)
- Also includes a general notes area using the markdown editor (saves to attempt.exercise_notes via autosave)
- All changes trigger the dirty flag for autosave

### Checklist Tab

- Interactive checklist from TaskChecklist component
- Each item shows label, required indicator, and a checkbox
- Checking/unchecking calls `toggle_checklist_item`
- Progress bar at top: "4 of 7 completed"
- Required items have a visual indicator

### Bugs Tab

- Bug log form and list (full implementation in Phase 5, but the tab and basic UI structure should exist here)
- For now: shows placeholder "Bug logging — built in Phase 5"

### Evidence Tab

- Evidence upload and management (full implementation in Phase 5)
- For now: shows placeholder "Evidence locker — built in Phase 5"

### Quiz Tab

- Shows quiz questions one at a time
- Each question: question text, answer input (textarea for short_answer/code_prompt/reflection, radio buttons for multiple_choice), timer showing seconds remaining
- "Submit Answer" button per question
- After submitting, shows whether answer was correct (for multiple_choice) or marks as "submitted for self-review" (for other types)
- After all questions answered, shows results summary via `get_quiz_results`

### Footer Bar

Always visible at the bottom of the working screen:

- **Autosave indicator:** "Last saved 3s ago" / "Saving..." / "Unsaved changes" — updates in real time
- **Session timer:** Running clock in HH:MM:SS format, starts when attempt loads
- **Memory Rebuild button:** Opens the MemoryRebuildPanel as a modal or slide-over
- **Score Card button:** Opens ScoreCardForm as a modal
- **Submit Attempt button:** Calls `submit_attempt`, shows confirmation dialog first

### Memory Rebuild Panel

Opens as a modal overlay:

- Instructions: "Rebuild today's implementation from memory. No notes, no reference. You have {memory_rebuild_minutes} minutes."
- Countdown timer starting from configured minutes
- A blank CodeMirror editor for the rebuild attempt
- "I'm Done" button with two options: "Pass — I rebuilt it successfully" / "Fail — I couldn't complete it"
- Notes field for what was forgotten or struggled with
- On completion: calls `complete_memory_rebuild`, closes panel, updates attempt state

### Score Card Form

Opens as a modal:

- 5 slider inputs (or number inputs with +/- steppers):
  - Implementation Completeness: 0–40
  - Code Quality: 0–20
  - Accessibility / UX: 0–15
  - Performance / Resilience: 0–15
  - Quiz: 0–10
- Live total calculation displayed prominently
- Color-coded quality gate indicator:
  - Red (<70): "BLOCKED — Replay required"
  - Yellow (70–84): "PASSED — Room for improvement"
  - Green (85–94): "STRONG PASS"
  - Purple (95–100): "MASTERY"
- If memory rebuild failed, shows warning: "Memory rebuild failed — day will be marked BLOCKED regardless of score"
- "Submit Score" button → calls `submit_attempt`

### Autosave Implementation

```typescript
// Inside the attempt page component
let isDirty = $state(false);
let lastSaved = $state<Date | null>(null);

$effect(() => {
  const timer = setInterval(async () => {
    if (isDirty && attempt) {
      try {
        await autosaveAttempt(attempt.id, {
          exerciseNotes: exerciseNotes,
          codeSnapshot: codeSnapshot,
          whatBroke, whyBroke, howFixed,
          refactorTomorrow, dailySummary,
        });
        lastSaved = new Date();
        isDirty = false;
      } catch (err) {
        console.error('Autosave failed:', err);
      }
    }
  }, 5000);

  return () => clearInterval(timer);
});
```

Every text change in any field sets `isDirty = true`. The timer checks every 5 seconds and saves if dirty.

### Resume Support

When navigating to the attempt page, it first calls `get_current_attempt(dayPlanId)`. If an in-progress attempt exists, it loads that attempt and resumes. If not, it shows a "Start New Attempt" prompt. This handles the case where the user closes the app mid-session and comes back later.

---

## CodeMirror 6 Integration

The CodeMirror component must support:

- Language modes: JavaScript, TypeScript, HTML, CSS, JSON, Markdown, Rust, Python
- Theme: one-dark (default), with support for switching via settings later
- Line numbers
- Bracket matching
- Auto-indent
- Search/replace (Ctrl+F)
- Props: `value: string`, `language: string`, `readonly: boolean`, `onchange: (value: string) => void`
- The component must properly clean up on destroy (Svelte 5 `$effect` cleanup)
- Must handle multiple instances on the same page (exercise entries)

---

## Attempt History Page (`/programs/[programId]/days/[dayId]/history`)

Shows all attempts for a day plan:

- Table with columns: Attempt #, Date, Score (with breakdown tooltip), Status badge, Duration
- Click an attempt to see its full detail (exercise notes, quiz answers, bug logs, reflections)
- Score comparison: bar chart showing score progression across attempts
- If multiple attempts exist, show delta (score change from previous attempt)

---

## Verification Checklist

1. **Start an attempt:** On a day mission page with met dependencies, click "Start Attempt". Verify new attempt created, working screen loads with left/right split, timer starts.

2. **Write code:** Add an exercise entry block, select "javascript" language, type code. Verify CodeMirror renders with syntax highlighting and line numbers.

3. **Autosave:** Type something, wait 6 seconds. Close the browser tab. Reopen the app, navigate back to the same day. Verify `get_current_attempt` resumes the in-progress attempt with your code preserved.

4. **Complete checklist items:** Switch to Checklist tab. Check off items. Verify progress bar updates. Refresh page — verify checked state persists.

5. **Take quiz:** Switch to Quiz tab. Answer all questions. Submit each. Verify results show after all answered.

6. **Memory rebuild:** Click Memory Rebuild in footer. Verify timer starts counting down. Type some code. Click "Pass" or "Fail". Verify rebuild state saved on the attempt.

7. **Submit scores:** Open Score Card. Enter scores (e.g., 35/18/12/13/8 = 86). Verify quality gate shows "STRONG PASS". Submit. Verify attempt status changes to "passed". Verify timer stops.

8. **Blocked scenario:** Start new attempt on same day. Submit with total score 55. Verify status is "blocked".

9. **Mastery scenario:** Start new attempt. Submit with 38/19/15/14/10 = 96. Verify status is "mastery".

10. **Memory rebuild override:** Start new attempt. Pass memory rebuild = false. Submit score of 90. Verify status is "blocked" despite high score.

11. **Attempt history:** Navigate to history page. Verify all attempts show with correct data. Verify score comparison chart renders.

12. **Multiple exercise blocks:** Add 3 exercise blocks with different languages. Reorder them. Delete one. Verify all operations work.

13. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete working screen with split-pane layout
- CodeMirror 6 integrated with multiple language modes
- Autosave every 5 seconds with resume support
- Session timer tracking
- Interactive checklist with persistence
- Quiz execution with timing
- Memory rebuild mechanic with countdown timer
- Score card submission with quality gate logic
- Attempt history with score comparison
- Time log tracking (implementation + memory rebuild sessions)

## Next Phase

Phase 5 builds the Evidence Locker and Bug Log system — completing the working screen's remaining tabs.
