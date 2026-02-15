# BuildOps 40 — Phase 12: Settings, Theme, Polish & Final QA

## What This Phase Builds

The finishing layer. Settings page with all configurable options, dark/light theme switching, loading skeletons for every page, empty states for every view, error boundaries, toast notifications throughout, window state persistence, and a comprehensive QA pass to verify every feature works end-to-end.

---

## Dependencies on Phase 11

All previous phases (1-11) must be complete. Every feature must be functional.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/commands/settings.rs` — 3 commands: get_setting, set_setting, get_all_settings
2. Update `src-tauri/src/lib.rs` — register settings commands

### Frontend Layer
3. `src/routes/settings/+page.svelte` — complete settings page (replace placeholder)
4. `src/lib/stores/app.svelte.ts` — add theme management, settings cache
5. `src/lib/components/ui/Skeleton.svelte` — loading skeleton component
6. `src/lib/components/ui/ProgressBar.svelte` — horizontal progress bar
7. `src/lib/components/ui/ProgressRing.svelte` — circular progress indicator
8. `src/lib/components/ui/Dropdown.svelte` — dropdown menu (if not already built)
9. Update every page that loads data: add loading skeletons and error states
10. Update every list page: add empty states with contextual CTAs
11. Update root layout: add theme class management, error boundary wrapper
12. Update sidebar: add active program indicator, quick stats

---

## Settings Page (`/settings`)

**Sections:**

### Appearance
- **Theme:** Toggle between Dark / Light / System (follows macOS preference)
  - Switching theme: adds/removes `light` class on `<html>` element
  - Persists to settings DB
  - System mode: uses `window.matchMedia('(prefers-color-scheme: dark')` listener

- **Font size:** Slider from 12 to 20px (default 14)
  - Applies to the entire app via CSS variable `--font-size-base`

- **Editor theme:** Dropdown: One Dark (default), Light
  - Passed to CodeMirror instances

- **Sidebar:** Collapsed by default toggle

### Learning
- **Autosave interval:** Dropdown: 3s, 5s (default), 10s, 15s, 30s
- **Default session time:** Number input in minutes (default 60)
- **Memory rebuild time:** Number input in minutes (default 15)
- **Blocked threshold:** Number input (default 70, range 50-100)
- **Mastery threshold:** Number input (default 95, range 80-100)
- **Streak freezes per month:** Number input (default 2, range 0-5)
- **Spaced repetition:** Toggle enabled/disabled (default enabled)

### Notifications
- **Daily reminder:** Toggle enabled/disabled
- **Reminder time:** Time input (default 09:00)
  - Uses `tauri-plugin-notification` for native macOS notifications

### Data Management
- **Export all data:** Button → exports every program as a combined JSON archive
- **Rebuild search index:** Button → triggers `rebuild_search_index`, shows progress
- **Clear all data:** Button → confirmation dialog ("This will permanently delete ALL programs, attempts, scores, and files. Type 'DELETE' to confirm.") → deletes database and recreates → restart app
- **App data location:** Shows the path to app_data_dir (read-only, with "Open in Finder" button)

### API Keys
- **Anthropic API key:** Masked input showing current stored key
  - "Update" to change
  - "Delete" to remove
  - Shows connection test button that makes a minimal API call to verify the key works

### About
- App version: 0.1.0
- Build info
- "Made by Billy Ribeiro"

---

## Theme Implementation

**Theme switching uses CSS custom properties already defined in Phase 1:**

```typescript
// In app.svelte.ts or theme.ts
function applyTheme(theme: 'dark' | 'light' | 'system') {
  const html = document.documentElement;
  
  if (theme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    html.classList.toggle('light', !prefersDark);
  } else if (theme === 'light') {
    html.classList.add('light');
  } else {
    html.classList.remove('light');
  }
}
```

The `light` class on `<html>` switches all CSS variables from dark to light values (already defined in `app.css` from Phase 1).

On app load: read theme setting from DB, apply immediately before first render to prevent flash.

---

## Loading Skeletons

Every page that loads data must show skeletons during the loading state.

**Skeleton component:** A pulsing rectangle with rounded corners and animated gradient shimmer. Accepts `width`, `height`, and `class` props.

**Usage pattern:**

```svelte
{#if loading}
  <div class="space-y-4">
    <Skeleton class="h-8 w-64" />
    <Skeleton class="h-32 w-full" />
    <div class="grid grid-cols-3 gap-4">
      <Skeleton class="h-48" />
      <Skeleton class="h-48" />
      <Skeleton class="h-48" />
    </div>
  </div>
{:else if error}
  <div class="rounded-lg border border-accent-danger/30 bg-accent-danger/10 p-6 text-center">
    <Icon icon="ph:warning-bold" width="32" class="mx-auto mb-2 text-accent-danger" />
    <p class="text-text-primary">Something went wrong</p>
    <p class="mt-1 text-sm text-text-secondary">{error}</p>
    <button class="mt-4 ..." onclick={retry}>Try Again</button>
  </div>
{:else}
  <!-- Actual content -->
{/if}
```

**Pages that need skeletons:**
- Dashboard (all widgets)
- Programs list
- Program detail
- Day mission view
- Working screen
- Analytics dashboard (all charts)
- Search results
- Export center
- Import hub
- Settings (while loading current values)

---

## Empty States

Every list view must show a meaningful empty state with a contextual call-to-action.

| Page | Empty State Message | CTA |
|------|-------------------|-----|
| Programs list | "No programs yet. Create your first learning program to get started." | "Create Program" button |
| Module list (in program) | "No modules in this program. Add your first module to organize your days." | "Add Module" button |
| Day plan list (in module) | "No days in this module. Add your first day plan." | "Add Day" button |
| Attempt history | "No attempts yet. Start your first attempt to begin learning." | "Start Attempt" button |
| Evidence locker | "No evidence yet. Upload files, add links, or save code snippets." | "Upload File" button |
| Bug logs | "No bugs logged. When you encounter issues, log them here." | "Add Bug" button |
| Search results | "No results found for '{query}'. Try different keywords." | — |
| Analytics (no data) | "Not enough data to show analytics. Complete some day attempts first." | "Go to Dashboard" link |
| Weekly reviews | "No weekly reviews yet. Complete at least one week of learning to generate a review." | — |
| Badges | "No badges earned yet. Keep learning and you'll earn your first badge soon." | — |
| Import history | "No imports yet. Upload curriculum PDFs to auto-generate your learning plan." | "Start Import" button |

---

## Error Boundaries

Wrap the main content area with an error boundary that catches unhandled errors and shows a recovery UI instead of a blank screen.

```svelte
<!-- In +layout.svelte or a wrapper component -->
<svelte:boundary onerror={handleError}>
  {@render children()}
</svelte:boundary>
```

The error handler:
- Logs the error to console
- Shows a friendly error card: "Something unexpected happened. Please try refreshing."
- "Refresh" button that reloads the current route
- "Go Home" link back to dashboard

---

## Toast Notification Audit

Ensure toasts fire for every user action that modifies data:

| Action | Toast Type | Message |
|--------|-----------|---------|
| Create program | success | "Program created" |
| Update program | success | "Program updated" |
| Delete program | success | "Program deleted" |
| Create module | success | "Module added" |
| Delete module | success | "Module deleted" |
| Create day plan | success | "Day plan created" |
| Update day plan | success | "Day plan saved" |
| Delete day plan | success | "Day plan deleted" |
| Start attempt | info | "Attempt started — good luck!" |
| Autosave | none (indicator only) | — |
| Submit attempt | success | "Attempt submitted — score: {total}" |
| Upload artifact | success | "File uploaded" |
| Delete artifact | success | "Artifact deleted" |
| Add bug log | success | "Bug logged" |
| Export PDF | success | "PDF exported to {filename}" |
| Export JSON | success | "Data exported" |
| Import complete | success | "Program imported — {dayCount} days created" |
| Settings saved | success | "Settings saved" |
| Error (any) | error | Specific error message, persists until dismissed |

---

## Window State Persistence

Remember window size and position between sessions:

- On window close: save width, height, x, y to settings
- On app launch: read saved values and apply to window before showing
- Use Tauri window API: `appWindow.innerSize()`, `appWindow.outerPosition()`

---

## Final QA Checklist — Complete End-to-End Verification

This is the master checklist. Every item must pass before the app is considered complete.

### Foundation
- [ ] App launches on macOS 13+
- [ ] Database created in correct location with all tables
- [ ] All settings have default values
- [ ] Dark theme renders correctly
- [ ] Light theme renders correctly
- [ ] System theme follows macOS preference
- [ ] Sidebar navigation works for all routes
- [ ] Keyboard shortcuts work (⌘+K, ⌘+1-5, ⌘+B, ⌘+N)

### Programs & Modules
- [ ] Create a program with title, description, target days
- [ ] Edit program title and description
- [ ] Delete program with confirmation
- [ ] Duplicate program
- [ ] Create 3 modules with different colors
- [ ] Reorder modules via drag
- [ ] Delete a module

### Day Plans
- [ ] Create a day plan with all content fields filled
- [ ] Add 5 checklist items, reorder them, delete one
- [ ] Add 3 quiz questions (short_answer, multiple_choice, code_prompt)
- [ ] Add 4 concept tags from different domains
- [ ] Add a dependency on another day
- [ ] Attempt circular dependency — verify rejection
- [ ] Publish day plan
- [ ] Edit published plan — verify version increments
- [ ] Duplicate day plan
- [ ] Reorder day plans within a program

### Day Attempts
- [ ] Start attempt — working screen loads with split pane
- [ ] Write code in CodeMirror — verify syntax highlighting
- [ ] Create 3 exercise entry blocks with different languages
- [ ] Autosave fires after 5 seconds (check last_autosave timestamp)
- [ ] Close and reopen app — resume in-progress attempt
- [ ] Complete checklist items — progress bar updates
- [ ] Take quiz — answer all questions, see results
- [ ] Start memory rebuild — timer counts down
- [ ] Complete memory rebuild as passed
- [ ] Submit scores — quality gate correctly evaluated
- [ ] Post-submission: score summary, streak update, badges shown

### Evidence & Bug Logs
- [ ] Upload a file via file picker
- [ ] Upload a file via drag and drop
- [ ] Add a link artifact
- [ ] Add a code snippet artifact
- [ ] Add a markdown note artifact
- [ ] Open a file artifact in system default app
- [ ] Delete an artifact
- [ ] Log a bug with all fields
- [ ] Edit a bug log entry
- [ ] Delete a bug log entry

### Scoring & Progression
- [ ] Score < 70 → status "blocked"
- [ ] Score 70-94 → status "passed"
- [ ] Score 95+ → status "mastery"
- [ ] Memory rebuild failed + high score → status "blocked"
- [ ] Streak increments on consecutive days
- [ ] Streak resets after gap (without freeze)
- [ ] Streak freeze preserves streak for 1 missed day
- [ ] Badge "First Step" awarded on first submission
- [ ] Badge "Mastery Achieved" awarded on 95+ score
- [ ] Skill scores update after submission

### Spaced Repetition
- [ ] Spaced repetition entry created after submission
- [ ] Due reviews appear on dashboard
- [ ] Recording a review updates interval and next review date

### Dependencies
- [ ] Unmet prerequisite blocks "Start Attempt"
- [ ] Met prerequisite enables "Start Attempt"
- [ ] Recommended dependencies show as suggestions, don't block

### Analytics
- [ ] Dashboard renders with all widgets populated
- [ ] Skill radar chart displays correct domains
- [ ] Score trend line shows submission history
- [ ] Concept heatmap shows tag × score matrix
- [ ] Burndown chart shows ideal vs actual pace
- [ ] Time analytics shows planned vs actual
- [ ] Bug analytics shows category distribution
- [ ] Weekly review auto-generates with correct data
- [ ] Weekly review manual notes editable

### Search
- [ ] FTS search finds day plan content
- [ ] FTS search finds exercise notes
- [ ] FTS search finds bug log content
- [ ] Search results show highlighted snippets
- [ ] Filter by entity type works
- [ ] Filter by program works
- [ ] Command palette opens with ⌘+K
- [ ] Command palette quick actions work
- [ ] Command palette keyboard navigation works

### Export & Import
- [ ] Export program PDF — file generates, opens correctly
- [ ] Export weekly PDF — file generates correctly
- [ ] Export JSON — valid JSON with all data
- [ ] Import JSON — program created with all entities
- [ ] Export/Import round trip — data integrity preserved
- [ ] Export CSV — valid spreadsheet format
- [ ] Export day plan template — JSON with content only

### PDF Ingestion
- [ ] Upload PDF files on import page
- [ ] Pipeline progresses through all steps
- [ ] Review interface shows generated plan with editable fields
- [ ] Edit fields in review, changes persist
- [ ] Apply import creates functional program
- [ ] Imported day plans work as attempt targets
- [ ] Cancel import stops pipeline cleanly
- [ ] Retry failed import works

### Settings
- [ ] Theme toggle: dark → light → system
- [ ] Font size change applies globally
- [ ] Autosave interval change takes effect
- [ ] Threshold changes apply to scoring
- [ ] Rebuild search index completes
- [ ] Clear all data works (with confirmation)
- [ ] API key storage and retrieval works

### Polish
- [ ] Loading skeletons show on every data-loading page
- [ ] Empty states show on every empty list
- [ ] Error states show on failure with retry
- [ ] Toasts fire for all data mutations
- [ ] Window size/position persists between sessions
- [ ] No console errors in normal usage
- [ ] `pnpm check` — zero TypeScript errors
- [ ] `cargo clippy` — zero Rust errors

---

## What's Done After This Phase

Everything. The app is complete and production-ready.

- Full settings page with all configurable options
- Dark/light/system theme switching
- Loading skeletons on every page
- Empty states on every list
- Error boundaries preventing blank screens
- Toast notifications on every data mutation
- Window state persistence
- Complete QA verification across all features

---

## Final Build

```bash
# Production build
pnpm tauri build

# Output
# src-tauri/target/release/bundle/dmg/BuildOps 40_0.1.0_aarch64.dmg
# src-tauri/target/release/bundle/macos/BuildOps 40.app
```

Distribute the `.dmg` directly. User opens it, drags to Applications, launches. Done.
