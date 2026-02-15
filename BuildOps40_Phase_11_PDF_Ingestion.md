# BuildOps 40 — Phase 11: PDF Ingestion Pipeline

## What This Phase Builds

The AI-powered curriculum import system. Drop PDF files (or markdown/text files) onto the app, it extracts the content, sends it to the Anthropic API for intelligent structuring, auto-generates a complete program with modules, day plans, checklists, quizzes, concept tags, and dependencies — then lets you review and edit everything before applying.

---

## Dependencies on Phase 10

Phase 10 must be complete. The import infrastructure and file dialog commands must be operational.

---

## What Gets Built

### Rust Layer — New Dependencies

Add to `src-tauri/Cargo.toml`:

```toml
pdf-extract = "0.7"                                    # PDF text extraction
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }  # HTTP for Anthropic API
tiktoken-rs = "0.6"                                    # Token counting
```

### Rust Layer
1. Migration `021_create_import_jobs.sql` — import_jobs table
2. `src-tauri/src/db/models/import_job.rs` — ImportJob struct + queries
3. `src-tauri/src/services/pdf_extractor.rs` — PDF/MD/TXT text extraction with structure detection
4. `src-tauri/src/services/document_chunker.rs` — intelligent chunking by section boundaries respecting token limits
5. `src-tauri/src/services/ai_analyzer.rs` — Anthropic API client, system prompt, response parsing, multi-chunk merge
6. `src-tauri/src/services/plan_generator.rs` — AI JSON output → BuildOps entity structs with validation
7. `src-tauri/src/services/import_applier.rs` — writes reviewed plan to database in single transaction
8. `src-tauri/src/commands/import.rs` — 8 commands: start_import, get_import_job, get_import_preview, update_import_preview, apply_import, cancel_import, list_import_jobs, delete_import_job, retry_import
9. Update `src-tauri/src/lib.rs` — register all new commands

### Frontend Layer
10. `src/lib/types/import.ts` — ImportJob, GeneratedPlanPreview, ImportStatus types
11. `src/lib/commands/import.ts` — typed wrappers for all 8+ commands
12. `src/lib/components/import/FileUploadZone.svelte` — multi-file drag & drop for PDFs/MD/TXT
13. `src/lib/components/import/ApiKeyInput.svelte` — secure input for Anthropic API key with "Remember" option
14. `src/lib/components/import/ImportProgress.svelte` — step-by-step progress indicator with live status updates
15. `src/lib/components/import/ImportReview.svelte` — full review/edit interface for the generated plan
16. `src/lib/components/import/ModuleReviewCard.svelte` — editable module within review
17. `src/lib/components/import/DayPlanReviewCard.svelte` — expandable/editable day plan card with all fields
18. `src/lib/components/import/ChecklistReviewList.svelte` — editable checklist items
19. `src/lib/components/import/QuizReviewList.svelte` — editable quiz questions
20. `src/routes/import/+page.svelte` — import hub (replace placeholder)
21. `src/routes/import/[jobId]/+page.svelte` — import progress + review page
22. `src/routes/import/[jobId]/+page.ts`
23. Add "Import Curriculum" to sidebar and command palette

---

## Pipeline Architecture

The import runs as a background Tokio task. The frontend polls for status every 2 seconds.

### Pipeline Steps

```
Step 1: EXTRACTING
  - Copy uploaded files to app_data_dir/imports/{job_id}/
  - For each file: extract text + detect structure (headings, code blocks, lists)
  - For PDF: use pdf-extract crate
  - For MD: parse with pulldown-cmark
  - For TXT: heuristic line-based parsing
  - Output: ExtractedDocument with sections, code blocks, metadata

Step 2: ANALYZING
  - Count tokens using tiktoken-rs
  - Chunk document by section boundaries (max ~100k tokens per chunk)
  - For each chunk: call Anthropic API with curriculum analysis system prompt
  - System prompt instructs model to return structured JSON
  - If multiple chunks: run merge pass to unify into single program
  - Output: raw AI JSON response

Step 3: GENERATING
  - Parse AI JSON into BuildOps entity structs
  - Validate: sequential day numbers, no circular deps, reasonable time estimates
  - Auto-fix where possible (renumber, clamp values)
  - Log validation warnings
  - Output: GeneratedPlan with all entities ready for review

Step 4: REVIEW (paused — waiting for user)
  - Frontend shows full review interface
  - User can edit everything: titles, content, checklists, quizzes, tags, deps
  - User clicks "Apply" when satisfied

Step 5: APPLYING
  - Write all entities to database in single SQLite transaction
  - Create program, modules, day plans, checklists, quizzes, tags, deps
  - Create spaced repetition entries
  - FTS triggers auto-index new content
  - Output: created Program

Step 6: COMPLETED
  - Navigate user to the new program
```

### API Call Details

**Model:** `claude-sonnet-4-20250514`
**Max tokens:** 8192 per response
**Temperature:** 0.3 (low for structured output consistency)

**System prompt** instructs the model to analyze educational content and return a complete program structure as JSON. The prompt specifies the exact JSON schema expected, rules for day plan content quality, minimum checklist/quiz requirements, concept tag granularity, and dependency logic. (Full prompt text is in the BuildOps40_PDF_Ingestion_Pipeline.md spec.)

**API key storage:** Encrypted in the settings table using AES-256-GCM with a device-derived key. For the initial implementation, storing the key in settings as plaintext is acceptable — encryption can be added as a polish item.

### Error Recovery

| Failure | Recovery |
|---------|----------|
| PDF extraction fails | Set status 'failed', show error, allow retry |
| API rate limited | Exponential backoff: 1s → 2s → 4s, max 5 retries |
| API returns invalid JSON | Retry with stricter prompt instruction, max 3 retries |
| API key invalid/missing | Fail immediately, prompt for new key |
| Network unreachable | Save progress, set 'failed', allow retry when online |
| Generated plan has validation issues | Show warnings in review UI, let user fix manually |
| Apply transaction fails | Full rollback, stay in 'review' status, allow retry |

---

## Import Hub Page (`/import`)

**Layout:**

1. **Upload zone** (top half)
   - Large FileUploadZone accepting .pdf, .md, .txt files
   - Multiple file support — shows list of selected files with sizes
   - Reorder files by drag

2. **Configuration** (below upload zone)
   - Target: "Create new program" (default) or "Add to existing program" (program selector)
   - API Key input (ApiKeyInput component)
   - "Remember API key" checkbox
   - "Start Import" button — disabled until files uploaded AND API key entered

3. **Past imports** (bottom)
   - Table: job ID, date, file names, status badge, actions (view/delete/retry)
   - Click a completed import to view the resulting program

---

## Import Review Interface (`/import/[jobId]`)

**When status is extracting/analyzing/generating:**
- ImportProgress component showing current step with animated indicator
- Step descriptions: "Extracting text from 4 PDF files...", "Analyzing curriculum structure with AI...", "Generating 14 modules and 40 day plans..."
- Elapsed time counter
- Cancel button

**When status is review:**
- Full scrollable review interface
- Program header: editable title and description
- Module list: each module is a card, expandable
  - Module title, color, description (all editable)
  - Inside each module: list of DayPlanReviewCards
    - Each card expandable to show ALL fields:
      - Title (text input)
      - Day number (number input)
      - Syntax targets (markdown editor)
      - Implementation brief (markdown editor)
      - Files to create (editable text)
      - Success criteria (markdown editor)
      - Stretch challenge (markdown editor)
      - Notes (markdown editor)
      - Estimated time (number input)
      - Checklist items (add/remove/edit list)
      - Quiz questions (add/remove/edit, with type selector)
      - Concept tags (tag selector)
      - Dependencies (day selector)
    - Delete day button
  - Add day button within module
- Add module button
- Summary bar: "14 modules, 40 days, 156 checklist items, 89 quiz questions, 45 concept tags"
- Validation warnings displayed at top (if any)
- "Apply to Program" button (primary action)
- "Save Draft" button (saves reviewed_plan_json for later)
- "Cancel Import" button

---

## Verification Checklist

1. **Upload PDFs:** Drag the 4 Svelte 5 curriculum PDFs onto the import zone. Verify all 4 appear in the file list with sizes.

2. **Start import:** Enter API key, click Start Import. Verify: status moves through extracting → analyzing → generating → review.

3. **Progress updates:** During pipeline execution, verify the progress indicator updates every 2 seconds with current step.

4. **Review generated plan:** When status hits 'review', verify: modules and day plans are displayed, content is populated, checklists and quizzes exist.

5. **Edit in review:** Change a day plan title. Remove a quiz question. Add a checklist item. Verify changes persist when scrolling away and back.

6. **Apply import:** Click "Apply to Program". Verify: program created, navigate to it, all modules and days present, content matches what was shown in review.

7. **Day plans functional:** Navigate to a day plan created by import. Verify: mission view renders all content, checklist items show, quiz questions exist, concept tags display.

8. **Start attempt on imported day:** Start an attempt on an imported day plan. Verify: working screen loads with the imported content in the left pane.

9. **Cancel import:** Start a new import. Cancel during the analyzing step. Verify: job marked as cancelled, no data created.

10. **Retry failed import:** Simulate a failure (use invalid API key). Verify error shown. Enter valid key. Click retry. Verify pipeline resumes.

11. **API key remembered:** Check "Remember API key". Close app. Reopen. Start new import. Verify API key is pre-filled.

12. **Multi-file import:** Upload 3 different PDFs. Verify they're treated as a unified curriculum with content from all files.

13. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete PDF/MD/TXT ingestion pipeline
- Anthropic API integration for AI-powered curriculum analysis
- Multi-file import with unified program generation
- Full review/edit interface before applying
- Background processing with progress polling
- Error recovery with retry capability
- API key storage

## Next Phase

Phase 12 builds Settings, Theme Switching, and Final Polish — the last phase before the app is complete.
