# BuildOps 40 — Phase 5: Evidence Locker & Bug Logs

## What This Phase Builds

Completes the two remaining tabs on the working screen: Evidence (file uploads, links, code snippets, notes) and Bug Logs (symptom/cause/fix/prevention tracking). Also builds the standalone Evidence Locker page that shows all artifacts across all attempts for a day.

---

## Dependencies on Phase 4

Phase 4 must be complete. The working screen must be operational with the Exercise and Checklist tabs functional.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/db/models/evidence.rs` — Artifact struct + all queries
2. `src-tauri/src/db/models/bug_log.rs` — BugLog struct + all queries
3. `src-tauri/src/commands/evidence.rs` — 7 commands for artifact management
4. `src-tauri/src/commands/bugs.rs` — 4 commands for bug log management (split from attempts.rs for clarity, or keep in attempts.rs — either pattern is fine as long as all commands are registered)
5. `src-tauri/src/utils/fs.rs` — file copy helper that copies uploaded files to app_data_dir/artifacts/{attempt_id}/
6. Update `src-tauri/src/lib.rs` — register new commands

### Frontend Layer
7. `src/lib/types/evidence.ts` — Artifact type definitions
8. `src/lib/types/bug.ts` — BugLog type definitions
9. `src/lib/commands/evidence.ts` — typed invoke wrappers
10. `src/lib/commands/bugs.ts` — typed invoke wrappers
11. `src/lib/components/ui/FileDropZone.svelte` — drag-and-drop file upload zone with visual feedback
12. `src/lib/components/evidence/EvidenceLocker.svelte` — full evidence management panel
13. `src/lib/components/evidence/ArtifactUploader.svelte` — file upload with progress
14. `src/lib/components/evidence/LinkAttacher.svelte` — URL input form
15. `src/lib/components/evidence/ScreenshotViewer.svelte` — inline image preview
16. `src/lib/components/evidence/BugLogEntry.svelte` — single bug log entry form/display
17. Update working screen: replace Evidence tab placeholder with EvidenceLocker
18. Update working screen: replace Bug tab placeholder with bug log list and form
19. `src/routes/programs/[programId]/days/[dayId]/evidence/+page.svelte` — standalone evidence page across all attempts
20. `src/routes/programs/[programId]/days/[dayId]/evidence/+page.ts`

---

## Evidence Commands (7)

| Command | Description |
|---------|-------------|
| `upload_artifact` | Takes attempt_id + file_path (from native file picker). Copies file to app_data_dir/artifacts/{attempt_id}/{uuid}_{original_filename}. Stores metadata (path, name, size, mime_type) in artifacts table. Returns created Artifact. |
| `add_link_artifact` | Creates artifact with type 'link', stores URL, title, description. |
| `add_code_artifact` | Creates artifact with type 'code_snippet', stores code_content and code_language. |
| `add_note_artifact` | Creates artifact with type 'markdown_note', stores markdown_content. |
| `delete_artifact` | Deletes artifact row. If file-based, also deletes the physical file from disk. |
| `list_artifacts` | Lists all artifacts for an attempt. Ordered by created_at desc. |
| `open_artifact_file` | For file-based artifacts, opens the file in the macOS default application using `tauri-plugin-shell` open command. |

### File Upload Flow

1. Frontend calls Tauri dialog plugin to open native file picker: `await open({ multiple: true })`
2. User selects files — dialog returns array of file paths
3. For each file, frontend calls `upload_artifact(attemptId, filePath, title, description)`
4. Rust command copies file to structured storage path, creates DB record
5. Frontend receives Artifact object, updates the list

### Drag and Drop

The FileDropZone component handles drag events on the Evidence tab:
- `ondragover` + `ondragenter`: visual highlight (border glow, text change)
- `ondrop`: extracts file paths, calls upload for each
- Since Tauri handles native file drops differently than web, the component must use Tauri's file drop events if available, falling back to web API

---

## Bug Log Commands (4)

| Command | Description |
|---------|-------------|
| `add_bug_log` | Creates bug log entry with symptom (required), root_cause, fix, prevention_rule, severity (low/medium/high/critical), category (freeform text). Returns created BugLog. |
| `update_bug_log` | Updates any provided fields. |
| `delete_bug_log` | Removes bug log entry. |
| `list_bug_logs` | Lists all bug logs for an attempt. Ordered by created_at desc. |

---

## Evidence Tab UI (in working screen)

**Layout:**
- FileDropZone at top: large dashed border area, "Drop files here or click to upload"
- Below the drop zone, 4 quick-add buttons in a row: "Upload File", "Add Link", "Add Code Snippet", "Add Note"
- Clicking each opens a small inline form:
  - Upload File: opens native file picker
  - Add Link: URL input + title input + optional description
  - Add Code Snippet: language selector + CodeMirror editor + title
  - Add Note: markdown editor + title
- Below the quick-add area: artifact list
  - Each artifact renders based on type:
    - File: icon (based on mime_type) + filename + size + "Open" button
    - Screenshot/image: inline thumbnail preview + filename + "Open" button
    - Link: clickable URL + title + description
    - Code snippet: syntax-highlighted code block (read-only CodeMirror) + language badge
    - Note: rendered markdown
  - Each artifact has a delete button (with confirmation)
- Empty state: "No evidence yet. Upload your first file or add a link."

---

## Bug Log Tab UI (in working screen)

**Layout:**
- "Add Bug" button at top
- Clicking opens inline form with fields:
  - Symptom (textarea, required) — "What happened?"
  - Root Cause (textarea) — "Why did it happen?"
  - Fix (textarea) — "How did you fix it?"
  - Prevention Rule (textarea) — "How to prevent it next time?"
  - Severity selector: Low / Medium / High / Critical (radio buttons or segmented control)
  - Category (text input with autocomplete from previous categories)
  - Save / Cancel buttons
- Below the form: list of bug entries for this attempt
  - Each entry shows severity badge (color-coded), symptom text, expand to see full details
  - Edit and delete buttons on each entry
- Bug count badge on the tab header: "Bugs (3)"
- Empty state: "No bugs logged. That's either great or suspicious."

---

## Standalone Evidence Page (`/programs/[programId]/days/[dayId]/evidence`)

Shows ALL artifacts across ALL attempts for this day plan:
- Filter dropdown: "All Attempts" / "Attempt 1" / "Attempt 2" / etc.
- Filter by type: All / Files / Links / Code / Notes
- Gallery/list view toggle
- Each artifact shows which attempt it belongs to
- This page is linked from the day mission view

---

## Verification Checklist

1. **Upload a file:** In the working screen Evidence tab, click "Upload File". Select a PDF. Verify file appears in the list with correct name and size. Click "Open" — verify it opens in Preview.app.

2. **Drag and drop:** Drag an image file onto the drop zone. Verify upload succeeds and thumbnail preview renders inline.

3. **Add a link:** Add a GitHub repo URL with title "Day 5 repo". Verify link artifact appears and URL is clickable.

4. **Add code snippet:** Add a JavaScript snippet with title "Grid layout component". Verify syntax highlighting renders.

5. **Add a note:** Add a markdown note with headers and code blocks. Verify rendered markdown displays correctly.

6. **Delete an artifact:** Delete the link artifact. Confirm deletion. Verify it's removed.

7. **Log a bug:** Switch to Bug tab. Add a bug: symptom "Grid items overlapping on mobile", root cause "Missing min-width on grid children", fix "Added minmax(0, 1fr) to grid-template-columns", prevention "Always use minmax in grid columns", severity High, category "CSS Layout". Save. Verify entry appears.

8. **Edit a bug:** Edit the severity to Medium. Save. Verify update persists.

9. **Multiple bugs:** Add 3 more bugs. Verify bug count badge on tab shows "Bugs (4)".

10. **Evidence across attempts:** Create a second attempt for the same day. Upload a different file. Navigate to the standalone evidence page. Verify both attempts' artifacts show. Filter to "Attempt 1" only — verify filtering works.

11. **File persistence:** Close and reopen the app. Navigate to the evidence page. Verify all files and artifacts are still present and accessible.

12. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete evidence management: file upload, links, code snippets, markdown notes
- Native file picker integration via Tauri dialog plugin
- File storage in structured directory under app_data_dir
- Bug log CRUD with severity and category tracking
- Both remaining working screen tabs fully operational
- Standalone evidence viewer across attempts
- All 5 working screen tabs now functional

## Next Phase

Phase 6 builds the Scoring Engine, Streak Tracking, and Badge System — the intelligence layer that turns raw data into progression metrics.
