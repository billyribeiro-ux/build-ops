# BuildOps 40 — Phase 9: Search & Command Palette

## What This Phase Builds

Full-text search across every piece of content in the app — day plans, exercise notes, bug logs, reflections, code snippets, artifacts. Also builds the Command Palette (⌘+K) for keyboard-driven navigation and quick actions.

---

## Dependencies on Phase 8

Phase 8 must be complete. All content-generating features must be operational.

---

## What Gets Built

### Rust Layer
1. Add FTS5 sync triggers to migration 018 (create a new migration 020 that adds the triggers, since we can't modify already-run migrations): triggers on day_plans, exercise_entries, bug_logs, day_attempts, artifacts
2. `src-tauri/src/services/search_index.rs` — FTS5 index management, rebuild logic
3. `src-tauri/src/commands/search.rs` — 2 commands: search, rebuild_search_index
4. Update `src-tauri/src/lib.rs` — register search commands

### Frontend Layer
5. `src/lib/types/search.ts` — SearchResult type definitions
6. `src/lib/commands/search.ts` — typed wrappers
7. `src/lib/components/search/GlobalSearch.svelte` — search input with results dropdown
8. `src/lib/components/search/SearchResults.svelte` — grouped results display
9. `src/lib/components/search/SearchFilters.svelte` — entity type and program filters
10. `src/lib/components/layout/CommandPalette.svelte` — ⌘+K modal with fuzzy search
11. `src/lib/utils/keyboard.ts` — global keyboard shortcut registration
12. `src/routes/search/+page.svelte` — full search page (replace placeholder)
13. Update `src/routes/+layout.svelte` — add command palette and global keyboard listeners
14. Update `src/lib/components/layout/TopBar.svelte` — wire search trigger to command palette

---

## FTS5 Trigger Migration

Create `src-tauri/src/db/migrations/020_add_fts_triggers.sql`:

Contains all INSERT/UPDATE/DELETE triggers for: day_plans, exercise_entries, bug_logs, day_attempts, artifacts. These keep the `search_index` FTS5 table in sync automatically. (The exact SQL was specified in the Phase 1 spec — migration 018. Since those triggers couldn't be created until the source tables had data flowing, they're added here.)

Also includes a one-time rebuild to index existing data:

```sql
-- Rebuild: index all existing data
INSERT INTO search_index(entity_id, entity_type, title, content, tags)
SELECT id, 'day_plan', title, 
    syntax_targets || ' ' || implementation_brief || ' ' || notes, ''
FROM day_plans;

-- ... same for exercise_entries, bug_logs, day_attempts, artifacts
```

---

## Search Command

```rust
#[tauri::command]
pub async fn search(
    state: State<'_, AppState>,
    query: String,
    entity_types: Option<Vec<String>>,
    program_id: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<SearchResult>, String>
```

**Logic:**
1. Sanitize query (escape FTS5 special characters)
2. Run FTS5 query: `SELECT entity_id, entity_type, snippet(search_index, 3, '<mark>', '</mark>', '...', 32) as snippet, rank FROM search_index WHERE search_index MATCH ? ORDER BY rank LIMIT ?`
3. If `entity_types` filter provided, add `AND entity_type IN (...)` 
4. For each result, join back to the source table to get: title, parent references (which program, which day, which attempt), creation date
5. If `program_id` filter provided, filter results to entities belonging to that program
6. Return results grouped by entity_type

**SearchResult type:**

```rust
pub struct SearchResult {
    pub entity_id: String,
    pub entity_type: String,      // day_plan, exercise, bug_log, attempt, artifact
    pub title: String,
    pub snippet: String,           // Highlighted text match
    pub program_title: Option<String>,
    pub day_plan_title: Option<String>,
    pub day_number: Option<i32>,
    pub created_at: String,
    pub relevance_score: f64,
    pub url: String,               // Frontend route to navigate to
}
```

The `url` field is constructed in Rust based on entity_type:
- day_plan → `/programs/{program_id}/days/{day_plan_id}`
- exercise → `/programs/{program_id}/days/{day_plan_id}/attempt` (links to the attempt that contains the exercise)
- bug_log → same as exercise
- attempt → `/programs/{program_id}/days/{day_plan_id}/history`
- artifact → `/programs/{program_id}/days/{day_plan_id}/evidence`

---

## Command Palette

Triggered by `⌘+K` globally. A modal overlay with:

1. **Search input** at top — auto-focused, debounced (300ms)
2. **Quick actions section** (shown when input is empty):
   - "New Day Plan" → navigates to day plan creator
   - "Start Today's Mission" → navigates to current day's attempt page
   - "Generate Weekly Review" → triggers review generation
   - "Open Settings" → navigates to settings
   - "Toggle Theme" → switches dark/light
3. **Recent pages section** (shown when input is empty):
   - Last 5 visited routes (tracked in frontend state)
4. **Search results** (shown when typing):
   - Calls `search` command with debounced input
   - Results grouped by entity_type
   - Each result shows icon + title + snippet + breadcrumb path
5. **Keyboard navigation:**
   - Arrow Up/Down to move selection
   - Enter to navigate to selected result
   - Escape to close
   - Type to filter

### Keyboard Shortcut System

Register global shortcuts in the root layout:

| Shortcut | Action |
|----------|--------|
| `⌘+K` | Toggle command palette |
| `⌘+N` | New day plan (navigates to creator) |
| `⌘+B` | Toggle sidebar collapsed/expanded |
| `⌘+1` through `⌘+5` | Navigate to Dashboard, Programs, Analytics, Search, Export |
| `⌘+Shift+T` | Start/stop session timer (when in attempt page) |
| `Escape` | Close command palette / close modal |

Implementation: single `keydown` listener on `window` in root layout. Check `e.metaKey` (⌘ on macOS) and `e.key`. Prevent default for handled shortcuts. Don't intercept when focus is in a text input or CodeMirror editor (except ⌘+K which should always work).

---

## Full Search Page (`/search`)

For users who want more than the command palette:

- Full-width search input at top
- Filter bar: entity type checkboxes (Day Plans, Exercises, Bug Logs, Attempts, Artifacts) + program selector dropdown
- Results list: each result is a card with icon, title, snippet (with `<mark>` highlighting), breadcrumb (Program > Module > Day), date
- Click result navigates to the source page
- Result count: "Found 23 results for 'grid template'"
- Empty state when no results: "No results found. Try different keywords."
- Pagination or infinite scroll for large result sets

---

## Verification Checklist

1. **Search finds day plans:** Create a day plan with "CSS Grid Template Areas" in the title. Search for "grid template". Verify the day plan appears in results.

2. **Search finds exercise notes:** Write exercise notes containing "flexbox align-items center". Search for "flexbox align". Verify the exercise result appears.

3. **Search finds bug logs:** Log a bug with symptom "overflow hidden clipping border radius". Search for "overflow border radius". Verify result.

4. **Search finds reflections:** Submit an attempt with daily_summary containing "finally understood reactive declarations". Search for "reactive declarations". Verify result.

5. **Search snippets highlight:** Verify the snippet in results contains `<mark>` tags around matching terms (rendered as highlighted text in the UI).

6. **Filter by type:** Search for a common term. Filter to "Bug Logs" only. Verify only bug log results show.

7. **Filter by program:** With 2 programs, search for a term that exists in both. Filter to one program. Verify only that program's results show.

8. **Command palette opens:** Press ⌘+K. Verify modal appears with focused search input.

9. **Command palette quick actions:** With empty input, verify quick actions show. Click "Open Settings". Verify navigation.

10. **Command palette keyboard nav:** Type a search term. Use arrow keys to select a result. Press Enter. Verify navigation.

11. **Keyboard shortcuts:** Press ⌘+1. Verify navigation to dashboard. Press ⌘+B. Verify sidebar toggles.

12. **Rebuild index:** Call `rebuild_search_index`. Verify it completes and returns count of indexed entries.

13. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Full-text search across all content types
- FTS5 triggers keeping index in sync automatically
- Command palette with fuzzy search + quick actions + keyboard navigation
- Global keyboard shortcuts for navigation
- Dedicated search page with filters
- Search result navigation to source content

## Next Phase

Phase 10 builds Export and Import — PDF reports, JSON data export/import, CSV export.
