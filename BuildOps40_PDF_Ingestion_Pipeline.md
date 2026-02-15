# BuildOps 40 — PDF Ingestion Pipeline

## Add-On Implementation Plan

### Assumes: All core BuildOps 40 features are built and operational

---

## OVERVIEW

A PDF ingestion system that accepts curriculum documents (PDFs, markdown files, text files) and automatically generates fully populated day plans, modules, checklist items, quiz questions, concept tags, syntax targets, dependencies, and estimated time allocations — ready to execute immediately.

---

## NEW DEPENDENCIES

### Rust (add to src-tauri/Cargo.toml)

```toml
pdf-extract = "0.7"          # PDF text extraction
lopdf = "0.34"                # Low-level PDF manipulation (fallback/metadata)
reqwest = { version = "0.12", features = ["json", "rustls-tls"] }  # HTTP client for Anthropic API
tiktoken-rs = "0.6"          # Token counting for chunking
```

### Frontend (add to package.json)

```json
{
  "pdfjs-dist": "^4.9.0"
}
```

No new frontend dependencies beyond pdfjs-dist for client-side PDF preview during the import review step.

---

## DATABASE ADDITIONS

### Migration 020 — Import Jobs

```sql
CREATE TABLE IF NOT EXISTS import_jobs (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT REFERENCES programs(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'extracting', 'analyzing', 'generating', 'review', 'applying', 'completed', 'failed')),
    source_type TEXT NOT NULL CHECK (source_type IN ('pdf', 'markdown', 'text', 'multi_file')),
    
    -- Source files
    source_files_json TEXT NOT NULL DEFAULT '[]',
    -- JSON array: [{ "file_name": "...", "file_path": "...", "file_size": 0 }]
    
    -- Extracted raw text
    extracted_text TEXT NOT NULL DEFAULT '',
    extracted_sections_json TEXT NOT NULL DEFAULT '[]',
    -- JSON array: [{ "heading": "...", "level": 1, "content": "...", "page": 1 }]
    
    -- AI analysis results
    ai_analysis_json TEXT NOT NULL DEFAULT '{}',
    -- Full structured output from Anthropic API
    
    -- Generated plan (pending user review)
    generated_plan_json TEXT NOT NULL DEFAULT '{}',
    -- Complete program structure ready to be applied
    
    -- User modifications during review
    reviewed_plan_json TEXT,
    
    -- Metrics
    total_pages INTEGER NOT NULL DEFAULT 0,
    total_tokens INTEGER NOT NULL DEFAULT 0,
    total_days_generated INTEGER NOT NULL DEFAULT 0,
    ai_model_used TEXT NOT NULL DEFAULT 'claude-sonnet-4-20250514',
    
    -- Error tracking
    error_message TEXT,
    error_step TEXT,
    
    -- Timing
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_import_jobs_status ON import_jobs(status);
CREATE INDEX idx_import_jobs_program ON import_jobs(program_id);
```

---

## RUST BACKEND — NEW MODULES

### File Structure

```
src-tauri/src/
├── commands/
│   └── import.rs              # Tauri IPC commands for import pipeline
├── services/
│   ├── pdf_extractor.rs       # PDF text + structure extraction
│   ├── document_chunker.rs    # Intelligent content chunking for AI
│   ├── ai_analyzer.rs         # Anthropic API integration
│   ├── plan_generator.rs      # Converts AI output → BuildOps entities
│   └── import_applier.rs      # Writes generated plan to database
```

### Service 1 — PDF Extractor (`services/pdf_extractor.rs`)

**Responsibility:** Takes raw file bytes, extracts text with structural metadata.

**Input:** File path (PDF, .md, .txt)

**Output:**

```rust
pub struct ExtractedDocument {
    pub file_name: String,
    pub total_pages: usize,
    pub raw_text: String,
    pub sections: Vec<ExtractedSection>,
    pub code_blocks: Vec<CodeBlock>,
    pub metadata: DocumentMetadata,
}

pub struct ExtractedSection {
    pub heading: String,
    pub level: u8,           // 1-6 for heading depth
    pub content: String,
    pub page_number: usize,
    pub has_code: bool,
    pub has_list: bool,
    pub estimated_complexity: u8,  // 1-5 heuristic
}

pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
    pub context_heading: String,  // Which section it belongs to
    pub page_number: usize,
}

pub struct DocumentMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub page_count: usize,
    pub word_count: usize,
    pub detected_languages: Vec<String>,  // Programming languages found in code blocks
    pub detected_topics: Vec<String>,     // Heuristic keyword extraction
}
```

**Logic:**
1. For PDF: use `pdf-extract` to pull text per page, detect headings by font size/weight heuristics, identify code blocks by monospace font detection or indentation patterns
2. For Markdown: parse with `pulldown-cmark`, extract heading hierarchy and fenced code blocks directly
3. For plain text: split on blank-line-separated sections, infer headings from ALL CAPS or numbered lines
4. Run language detection on code blocks (pattern matching for `fn`, `const`, `def`, `<script>`, CSS selectors, etc.)
5. Compute estimated complexity per section: presence of code (higher), length (longer = higher), nesting depth, technical keyword density

### Service 2 — Document Chunker (`services/document_chunker.rs`)

**Responsibility:** Splits extracted content into optimally sized chunks for the AI API, respecting section boundaries.

**Input:** `ExtractedDocument`

**Output:**

```rust
pub struct ChunkedDocument {
    pub chunks: Vec<DocumentChunk>,
    pub total_tokens: usize,
    pub chunk_strategy: ChunkStrategy,
}

pub struct DocumentChunk {
    pub index: usize,
    pub content: String,
    pub token_count: usize,
    pub section_refs: Vec<usize>,  // Which sections are in this chunk
    pub is_continuation: bool,
}

pub enum ChunkStrategy {
    SinglePass,        // Document fits in one API call (< 150k tokens)
    SectionBased,      // Split by top-level sections
    MultiPass,         // Large doc requires multiple analysis passes + merge
}
```

**Logic:**
1. Count tokens using `tiktoken-rs` (cl100k_base encoding)
2. If total tokens < 150,000: single chunk, single API call
3. If 150k–500k: split by top-level sections (h1/h2 boundaries), each chunk ≤ 100k tokens
4. If > 500k: split by h2/h3, with overlap context (last paragraph of previous chunk prepended)
5. Never split mid-code-block or mid-paragraph
6. Each chunk includes a preamble: "This is chunk {n}/{total} of document '{title}'. Previous sections covered: {summary}."

### Service 3 — AI Analyzer (`services/ai_analyzer.rs`)

**Responsibility:** Sends chunked content to Anthropic API, receives structured curriculum analysis.

**API Configuration:**

```rust
pub struct AiConfig {
    pub api_key: String,              // Stored encrypted in settings
    pub model: String,                // claude-sonnet-4-20250514
    pub max_tokens: usize,            // 8192 per response
    pub temperature: f32,             // 0.3 for structured output
    pub base_url: String,             // https://api.anthropic.com/v1/messages
}
```

**System Prompt for Analysis:**

```
You are a curriculum architect. Given educational content, you produce a structured learning plan as JSON.

Analyze the provided content and generate a complete learning program with the following structure:

{
  "program_title": "string",
  "program_description": "string",
  "estimated_total_days": number,
  "modules": [
    {
      "title": "string",
      "description": "string",
      "order_index": number,
      "color": "hex string",
      "days": [
        {
          "day_number": number,
          "title": "string — specific and actionable",
          "syntax_targets": "markdown — exact syntax/concepts to master",
          "implementation_brief": "markdown — what to build, specific deliverables",
          "files_to_create": "markdown — list of files the learner should create",
          "success_criteria": "markdown — measurable criteria for completion",
          "stretch_challenge": "markdown — optional advanced extension",
          "notes": "markdown — tips, gotchas, references",
          "estimated_minutes": number,
          "memory_rebuild_minutes": number,
          "checklist_items": [
            { "label": "string — specific task", "is_required": boolean }
          ],
          "quiz_questions": [
            {
              "question_text": "string",
              "question_type": "short_answer | multiple_choice | code_prompt | reflection",
              "correct_answer": "string",
              "options": ["string"] | [],
              "points": number,
              "time_limit_seconds": number
            }
          ],
          "concept_tags": [
            { "name": "string", "domain": "string" }
          ],
          "dependencies": [
            { "depends_on_day_number": number, "type": "prerequisite | recommended", "minimum_score": number }
          ]
        }
      ]
    }
  ]
}

Rules:
- Each day should take 45-120 minutes for a focused learner.
- Day titles must be specific (not "Learn CSS" but "CSS Grid Layout + Template Areas").
- Syntax targets must include exact code patterns the learner should memorize.
- Implementation briefs must describe a concrete build output, not vague exercises.
- Every day gets 3-8 checklist items that are individually verifiable.
- Every day gets 2-5 quiz questions covering the day's core concepts.
- Concept tags use granular names (not "CSS" but "$state rune", "grid-template-columns", "async/await").
- Dependencies should be set when a day requires knowledge from an earlier day.
- Group days into logical modules (5-10 days each).
- Assign module colors from this palette: #6366F1, #EC4899, #F59E0B, #22C55E, #3B82F6, #A855F7, #EF4444, #14B8A6.
- Stretch challenges should be genuinely harder, not just "do more of the same".
- Quiz questions should test understanding, not just recall.
- Respond with ONLY the JSON object. No markdown fences, no preamble.
```

**Multi-Chunk Merge Strategy:**

When document requires multiple API calls:
1. First pass: Analyze each chunk independently, get per-chunk day plans
2. Merge pass: Send summaries of all chunks + all generated plans to a final API call with prompt: "Given these separately analyzed sections, produce a unified program. Deduplicate overlapping content, ensure day numbering is sequential, resolve cross-chunk dependencies, and ensure logical module grouping."
3. The merge pass returns the canonical `generated_plan_json`

**Error Handling:**
- Retry with exponential backoff (3 attempts, 1s → 2s → 4s)
- If API returns malformed JSON: retry with appended instruction "Your previous response was not valid JSON. Respond with ONLY a JSON object."
- If API is unreachable: save progress, set job status to `failed`, store error message
- Validate generated JSON against expected schema before proceeding

### Service 4 — Plan Generator (`services/plan_generator.rs`)

**Responsibility:** Transforms the AI's JSON output into BuildOps entity structs ready for database insertion.

**Input:** Parsed AI JSON response

**Output:**

```rust
pub struct GeneratedPlan {
    pub program: ProgramDraft,
    pub modules: Vec<ModuleDraft>,
    pub day_plans: Vec<DayPlanDraft>,
    pub checklist_items: Vec<ChecklistItemDraft>,
    pub quiz_questions: Vec<QuizQuestionDraft>,
    pub concept_tags: Vec<ConceptTagDraft>,
    pub tag_assignments: Vec<(usize, String)>,  // (day_index, tag_name)
    pub dependencies: Vec<DependencyDraft>,
    pub validation_warnings: Vec<String>,
}
```

**Validation logic:**
- Day numbers are sequential with no gaps
- All dependency references point to valid day numbers
- No circular dependencies (topological sort check)
- Estimated minutes are within 15–180 range
- Every day has at least 1 checklist item and 1 quiz question
- Concept tag names are normalized (lowercase, trimmed, deduplicated)
- Module order indices are sequential
- Total days matches sum of all module days

**If validation finds issues:** Auto-fix where possible (renumber days, clamp minutes, add default checklist item), log warnings for user review.

### Service 5 — Import Applier (`services/import_applier.rs`)

**Responsibility:** Writes the reviewed plan to the database as real entities, within a single transaction.

**Logic:**
1. Begin SQLite transaction
2. Create or update program (user may be importing into existing program)
3. Create modules in order
4. Create day plans with all content fields
5. Create checklist items per day plan
6. Create quiz questions per day plan
7. Create or find concept tags (dedup against existing tags)
8. Create tag assignments
9. Create dependencies (resolve day_number references to actual day_plan IDs)
10. Create initial spaced repetition entries for each day plan
11. Update search index (FTS5 triggers handle this automatically)
12. Commit transaction
13. If any step fails: rollback entire transaction, set job status to `failed`

---

## RUST IPC COMMANDS

```rust
// commands/import.rs

#[tauri::command]
async fn start_import(
    state: State<'_, AppState>,
    app: AppHandle,
    file_paths: Vec<String>,
    program_id: Option<String>,
    api_key: String,
) -> Result<ImportJob, String>;
// 1. Copies files to app_data_dir/imports/{job_id}/
// 2. Creates import_job record with status 'pending'
// 3. Spawns async task for the pipeline
// 4. Returns job immediately so frontend can poll

#[tauri::command]
async fn get_import_job(
    state: State<'_, AppState>,
    job_id: String,
) -> Result<ImportJob, String>;
// Returns current job status + progress for polling

#[tauri::command]
async fn get_import_preview(
    state: State<'_, AppState>,
    job_id: String,
) -> Result<GeneratedPlanPreview, String>;
// Returns the generated plan for user review (only when status = 'review')
// GeneratedPlanPreview includes all modules, days, checklists, quizzes
// with edit capabilities before applying

#[tauri::command]
async fn update_import_preview(
    state: State<'_, AppState>,
    job_id: String,
    reviewed_plan_json: String,
) -> Result<(), String>;
// User edits the generated plan — saves modifications

#[tauri::command]
async fn apply_import(
    state: State<'_, AppState>,
    job_id: String,
) -> Result<Program, String>;
// Takes the reviewed plan and writes it all to the database
// Returns the created/updated program

#[tauri::command]
async fn cancel_import(
    state: State<'_, AppState>,
    job_id: String,
) -> Result<(), String>;
// Cancels in-progress import, cleans up temp files

#[tauri::command]
async fn list_import_jobs(
    state: State<'_, AppState>,
) -> Result<Vec<ImportJobSummary>, String>;
// Returns all import jobs with status

#[tauri::command]
async fn delete_import_job(
    state: State<'_, AppState>,
    job_id: String,
) -> Result<(), String>;
// Deletes job record and associated temp files

#[tauri::command]
async fn retry_import(
    state: State<'_, AppState>,
    job_id: String,
    api_key: String,
) -> Result<ImportJob, String>;
// Retries a failed import from the last successful step
```

---

## ASYNC PIPELINE EXECUTION

The import runs as a background Tokio task so the UI stays responsive.

```rust
// Spawned from start_import command
tokio::spawn(async move {
    let job_id = job.id.clone();
    
    // Step 1: Extract
    update_job_status(&pool, &job_id, "extracting").await;
    let extracted = match extract_documents(&file_paths).await {
        Ok(doc) => doc,
        Err(e) => {
            fail_job(&pool, &job_id, "extracting", &e.to_string()).await;
            return;
        }
    };
    
    // Step 2: Chunk
    update_job_status(&pool, &job_id, "analyzing").await;
    let chunked = chunk_document(&extracted);
    
    // Step 3: AI Analysis
    let ai_result = match analyze_with_ai(&config, &chunked).await {
        Ok(result) => result,
        Err(e) => {
            fail_job(&pool, &job_id, "analyzing", &e.to_string()).await;
            return;
        }
    };
    
    // Step 4: Generate plan
    update_job_status(&pool, &job_id, "generating").await;
    let plan = match generate_plan(&ai_result) {
        Ok(plan) => plan,
        Err(e) => {
            fail_job(&pool, &job_id, "generating", &e.to_string()).await;
            return;
        }
    };
    
    // Step 5: Save for review
    save_generated_plan(&pool, &job_id, &plan).await;
    update_job_status(&pool, &job_id, "review").await;
    
    // Pipeline pauses here — user reviews in UI
    // apply_import command resumes when user approves
});
```

**Frontend polls `get_import_job` every 2 seconds while status is `extracting`, `analyzing`, or `generating`.** When status hits `review`, the UI transitions to the review/edit screen.

---

## FRONTEND — NEW COMPONENTS AND ROUTES

### Routes

```
src/routes/
├── import/
│   ├── +page.svelte              # Import hub — upload files, see past imports
│   ├── +page.ts
│   ├── [jobId]/
│   │   ├── +page.svelte          # Import progress + review screen
│   │   └── +page.ts
```

### Components

```
src/lib/components/import/
├── FileUploadZone.svelte          # Multi-file drag & drop (PDF, MD, TXT)
├── ImportProgress.svelte          # Step-by-step progress indicator with status per step
├── ImportReview.svelte            # Full review/edit interface for generated plan
├── ModuleReviewCard.svelte        # Editable module card within review
├── DayPlanReviewCard.svelte       # Editable day plan card — expand to see all fields
├── ChecklistReviewList.svelte     # Edit/add/remove checklist items
├── QuizReviewList.svelte          # Edit/add/remove quiz questions
├── TagReviewSelector.svelte       # Edit concept tag assignments
├── DependencyReviewGraph.svelte   # Visual dependency editor
├── ImportDiffView.svelte          # Shows what will be created when applied
└── ApiKeyInput.svelte             # Secure input for Anthropic API key (stored encrypted)
```

### Import Hub (`/import`)

**Layout:**
1. Upload zone (top) — drag & drop or file picker for PDFs/MD/TXT, supports multiple files
2. Configuration panel — target program (new or existing), API key input
3. "Start Import" button — disabled until files uploaded + API key entered
4. Past imports table (bottom) — list of previous import jobs with status, date, actions

### Import Progress (`/import/[jobId]`)

**Layout:**
1. Step progress bar: Upload → Extract → Analyze → Generate → Review → Apply
2. Current step detail: animated indicator, elapsed time, estimated remaining
3. Live log feed: shows what the pipeline is doing ("Extracting page 3 of 47...", "Sending chunk 2/4 to AI...", "Generated 12 day plans...")
4. When status = `review`: transitions to review interface

### Import Review Interface

**This is the critical screen.** User must be able to review and edit everything before it hits the database.

**Layout: Scrollable single page with collapsible sections**

1. **Program header** — edit title, description
2. **Module list** — reorder, rename, change colors, add/remove modules
3. **Per module: Day plan cards** — each expandable to show:
   - Title (editable)
   - Day number (editable, auto-resequences)
   - Syntax targets (markdown editor)
   - Implementation brief (markdown editor)
   - Files to create (editable list)
   - Success criteria (markdown editor)
   - Stretch challenge (markdown editor)
   - Notes (markdown editor)
   - Estimated time (number input)
   - Checklist items (add/remove/reorder/edit)
   - Quiz questions (add/remove/edit with type selector)
   - Concept tags (tag selector)
   - Dependencies (dropdown selector referencing other days)
4. **Summary footer** — total days, total modules, total quiz questions, total concept tags
5. **Action buttons** — "Apply to Program" (creates everything), "Save Draft" (saves reviewed_plan_json), "Cancel Import"

**Key UX details:**
- Expand/collapse all toggle
- Bulk operations: "Delete all stretch challenges", "Set all estimated times to 60 min"
- Validation indicators: red outlines on invalid fields, warning icons on days with no quiz questions
- Drag-and-drop reordering for days within modules and modules within program

---

## API KEY MANAGEMENT

The Anthropic API key is stored encrypted in the settings table.

```rust
// Encryption: AES-256-GCM using a device-derived key
// Key derivation: PBKDF2 from machine UUID + app salt
// Stored in settings as: { "anthropic_api_key": "<encrypted_base64>" }
```

**Frontend flow:**
1. First import: prompt for API key with secure input field
2. "Remember API key" checkbox — if checked, encrypts and stores
3. If stored: auto-fills on subsequent imports, shows masked value
4. Settings page: option to view (requires confirmation), update, or delete stored key

---

## SUPPORTED FILE TYPES

| Type | Extension | Extraction Method |
|------|-----------|------------------|
| PDF | .pdf | `pdf-extract` crate — text per page + font metadata |
| Markdown | .md | `pulldown-cmark` — native parsing |
| Plain Text | .txt | Line-by-line with heading inference |
| Multiple files | mixed | Each file extracted independently, concatenated with file boundary markers |

**Future consideration (not in this phase):** .docx (via `docx-rs`), .pptx, .epub, HTML.

---

## MULTI-FILE IMPORT

When multiple files are uploaded:
1. Each file is extracted independently
2. Files are ordered by name (user can reorder in UI before starting)
3. Extracted content is concatenated with clear boundary markers:
   ```
   === FILE: Svelte5_14-Day_Syntax_Roadmap.pdf (47 pages) ===
   [content]
   === FILE: Svelte5_Companion_Exercises.pdf (12 pages) ===
   [content]
   ```
4. The AI prompt includes: "This curriculum spans multiple source documents. Treat them as a unified program and resolve any overlapping content."

---

## ERROR RECOVERY

| Failure Point | Recovery |
|---------------|----------|
| PDF extraction fails | Mark job failed, show error with file name, allow retry with different file |
| API rate limited | Auto-retry with exponential backoff, up to 5 attempts |
| API returns invalid JSON | Retry with stricter prompt, up to 3 attempts |
| API key invalid | Fail immediately, prompt for new key |
| Network unreachable | Pause job, auto-retry when connectivity returns (check every 30s) |
| Generation produces invalid plan | Show validation warnings in review UI, let user fix manually |
| Apply transaction fails | Full rollback, job stays in `review` status, user can retry apply |

---

## IMPLEMENTATION ORDER

1. Migration 020 + ImportJob model
2. `pdf_extractor.rs` — PDF/MD/TXT extraction
3. `document_chunker.rs` — token counting + chunking
4. `ai_analyzer.rs` — Anthropic API client + prompt
5. `plan_generator.rs` — JSON → BuildOps entity mapping + validation
6. `import_applier.rs` — database transaction writer
7. `commands/import.rs` — all 8 IPC commands
8. Frontend: `FileUploadZone.svelte` + `ApiKeyInput.svelte`
9. Frontend: `/import` page with upload + config
10. Frontend: `ImportProgress.svelte` with polling
11. Frontend: Full review interface with all editable fields
12. Frontend: Apply flow with confirmation + success redirect
13. Frontend command wrappers in `$lib/commands/import.ts`
14. Add "Import Curriculum" to sidebar navigation + command palette
15. Integration test: upload the 4 Svelte 5 PDFs from your curriculum → verify generated plan matches expected structure

---

## WHAT "DONE" LOOKS LIKE

1. User drags 4 PDF files onto the import screen
2. Enters Anthropic API key (or auto-fills from stored)
3. Clicks "Start Import"
4. Watches real-time progress as pipeline extracts → analyzes → generates
5. Reviews the generated program: 14 modules with 40 day plans, each with syntax targets, implementation briefs, checklists, quizzes, concept tags, and dependencies
6. Edits a few day titles, adds a checklist item, removes a quiz question
7. Clicks "Apply to Program"
8. Navigates to the new program and sees all 40 days ready to execute
9. Day 1 mission screen has the exact syntax targets, checklist, and quiz from the PDF content
10. Full-text search finds content from the imported PDFs
11. Spaced repetition entries are created for every day
12. The whole process takes under 3 minutes for a 50-page curriculum

---

**END OF PDF INGESTION PIPELINE PLAN**
