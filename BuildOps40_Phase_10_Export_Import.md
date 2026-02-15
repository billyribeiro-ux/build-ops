# BuildOps 40 — Phase 10: Export & Import

## What This Phase Builds

Complete data portability. Export programs as PDF reports, JSON archives (re-importable), and CSV spreadsheets. Import programs from JSON to move data between machines or restore backups.

---

## Dependencies on Phase 9

Phase 9 must be complete. All data-generating features must be operational.

---

## What Gets Built

### Rust Layer — New Dependencies

Add to `src-tauri/Cargo.toml`:

```toml
genpdf = "0.3"            # PDF generation
csv = "1.3"               # CSV writing
```

### Rust Layer
1. `src-tauri/src/services/export_pdf.rs` — PDF report generation using genpdf
2. `src-tauri/src/services/export_data.rs` — JSON and CSV export logic
3. `src-tauri/src/services/import.rs` — JSON import with validation and transaction
4. `src-tauri/src/commands/export.rs` — 6 commands: export_program_pdf, export_weekly_report_pdf, export_program_json, export_program_csv, import_program_json, export_day_plan_template
5. `src-tauri/src/commands/files.rs` — 3 commands: pick_file, pick_save_location, get_app_data_dir
6. Update `src-tauri/src/lib.rs` — register all 9 new commands

### Frontend Layer
7. `src/lib/commands/export.ts` — typed wrappers
8. `src/lib/commands/files.ts` — typed wrappers for file dialogs
9. `src/routes/export/+page.svelte` — export center (replace placeholder)
10. Update import page to include JSON import capability (PDF ingestion is Phase 11)

---

## Export Commands (6)

### export_program_pdf
- Opens native save dialog for output path
- Generates a comprehensive PDF report containing:
  - Program title, description, dates, overall stats
  - Per-module summary: module title, day count, average score
  - Per-day summary: day number, title, score, status, time spent
  - Skill radar data as a table (charts can't render in genpdf, so present as tabular data)
  - Weekly review summaries
  - Overall analytics: total time, average score, streak history, badges earned
- PDF formatting: clean headers, tables with borders, section dividers, page numbers
- Uses `genpdf` crate with built-in fonts (no external font files needed)

### export_weekly_report_pdf
- Same save dialog flow
- Generates a focused PDF for a single week:
  - Week number, date range
  - Days completed that week with scores
  - Strong/weak concepts
  - Bug patterns
  - Replay recommendations
  - Manual notes and goals

### export_program_json
- Exports the COMPLETE program data as a single JSON file:
  - Program metadata
  - All modules with order
  - All day plans with ALL content fields
  - All checklist items per day plan
  - All quiz questions per day plan
  - All concept tags and assignments
  - All dependencies
  - All day attempts with scores, reflections, exercise notes
  - All bug logs
  - All exercise entries
  - All artifacts (metadata only — not the binary files)
  - All time logs
  - Skill scores
  - Weekly reviews
  - Streak data
  - Badges
- This is the canonical backup format. It can be re-imported.

**JSON structure:**
```json
{
  "export_version": "1.0",
  "exported_at": "2026-02-15T12:00:00Z",
  "app_version": "0.1.0",
  "program": { ... },
  "modules": [ ... ],
  "day_plans": [ ... ],
  "checklist_items": [ ... ],
  "quiz_questions": [ ... ],
  "concept_tags": [ ... ],
  "tag_assignments": [ ... ],
  "dependencies": [ ... ],
  "attempts": [ ... ],
  "exercise_entries": [ ... ],
  "bug_logs": [ ... ],
  "artifacts": [ ... ],
  "time_logs": [ ... ],
  "skill_scores": [ ... ],
  "weekly_reviews": [ ... ],
  "streak": { ... },
  "badges": [ ... ]
}
```

### export_program_csv
- Flat CSV export of attempt data for spreadsheet analysis:
  - Columns: day_number, day_title, module_title, attempt_number, date, score_implementation, score_code_quality, score_accessibility, score_performance, score_quiz, total_score, status, actual_minutes, memory_rebuild_passed
  - One row per attempt
  - Sorted by day_number then attempt_number

### import_program_json
- Opens native file picker for JSON file
- Validates JSON structure against expected schema
- Validates all required fields present
- Creates program + all child entities in a single SQLite transaction
- If program title already exists, appends " (imported)" to avoid conflicts
- Concept tags are deduped against existing tags by name
- Returns the newly created Program
- If any validation fails: returns error with specific details about what's wrong

### export_day_plan_template
- Exports a single day plan as a reusable template JSON:
  - Day plan content fields
  - Checklist items
  - Quiz questions
  - Concept tags (names only, not IDs)
- Does NOT include attempts, evidence, or scores
- Can be used as a starting point when creating new days

---

## File Dialog Commands (3)

| Command | Description |
|---------|-------------|
| `pick_file` | Opens native macOS file picker using `tauri-plugin-dialog`. Returns selected file path or null if cancelled. Supports file type filters. |
| `pick_save_location` | Opens native save dialog with default filename. Returns selected path or null. |
| `get_app_data_dir` | Returns the app data directory path for reference. |

---

## Export Center Page (`/export`)

**Layout:**

1. **Program selector** at top — dropdown to choose which program to export

2. **Export options** as cards:

   **PDF Program Report**
   - Description: "Comprehensive PDF report with all scores, analytics, and weekly reviews"
   - "Export PDF" button → opens save dialog → generates PDF → shows success toast

   **PDF Weekly Report**
   - Week number selector (dropdown of available weeks)
   - "Export PDF" button → same flow

   **JSON Full Backup**
   - Description: "Complete data export. Can be re-imported on another machine."
   - "Export JSON" button → save dialog → generates JSON → success toast
   - Shows estimated file size

   **CSV Scores**
   - Description: "Flat spreadsheet of all attempt scores for analysis"
   - "Export CSV" button → save dialog → generates CSV → success toast

   **Day Plan Template**
   - Day plan selector (dropdown of all days in selected program)
   - "Export Template" button → save dialog → generates template JSON

3. **Import section** at bottom:

   **Import Program from JSON**
   - Description: "Restore a program from a previous JSON export"
   - "Choose File" button → file picker (filtered to .json) → validates → shows preview of what will be imported (program title, day count, module count) → "Confirm Import" button → imports → navigates to new program

---

## PDF Generation Notes

`genpdf` is a pure-Rust PDF library. It uses built-in fonts (Helvetica, etc.) so no font files are needed.

**Report structure:**
- Title page: program title, date range, "Generated by BuildOps 40"
- Table of contents (manual, since genpdf doesn't auto-generate TOC)
- Executive summary: total days, completion %, average score, total time
- Module sections: each module gets a heading + table of its day results
- Analytics section: key metrics as formatted text and tables
- Weekly reviews section: each review as a formatted block
- Badges section: list of earned badges with dates

Tables use genpdf's `TableLayout` with column widths calculated from content. All text is properly encoded and escaped.

---

## Verification Checklist

1. **Export PDF:** Select a program with 5+ submitted days. Export PDF. Open the file. Verify: title page, summary stats, per-day table with scores, module groupings, page numbers.

2. **Export weekly PDF:** Export a specific week's report. Verify: correct date range, correct day data, strong/weak concepts listed.

3. **Export JSON:** Export a program. Open the JSON file. Verify: all expected top-level keys present, program data matches, day plans have content, attempts have scores.

4. **Import JSON:** Take the exported JSON. Import it. Verify: new program created with correct title (+ "(imported)" if name collision), all modules present, all day plans with content, all concept tags, all attempts with scores.

5. **Round-trip integrity:** Export program A as JSON. Import it as program B. Compare: same number of modules, days, attempts, scores. Data matches exactly.

6. **Export CSV:** Export scores CSV. Open in a spreadsheet app. Verify: correct columns, one row per attempt, scores match.

7. **Export template:** Export a day plan template. Verify: content fields present, checklists present, quizzes present, no attempt data.

8. **Import validation:** Modify an exported JSON to remove a required field. Try importing. Verify: error message indicating what's missing.

9. **File dialogs:** Verify native macOS file picker and save dialogs open correctly with proper file filters.

10. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- PDF report generation for programs and weekly reviews
- JSON full backup export (re-importable)
- CSV score export for spreadsheet analysis
- Day plan template export
- JSON import with validation and transaction safety
- Native file picker and save dialogs
- Export center UI

## Next Phase

Phase 11 builds the PDF Ingestion Pipeline — AI-powered curriculum import from PDF documents.
