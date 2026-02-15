# BuildOps 4.0 - PDF Ingestion Pipeline Implementation Guide

## 🎉 Implementation Complete

The PDF Ingestion Pipeline has been fully implemented and is ready for use. This document provides a comprehensive guide to the system.

---

## 📋 Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Backend Components](#backend-components)
4. [Frontend Components](#frontend-components)
5. [User Workflow](#user-workflow)
6. [API Reference](#api-reference)
7. [Configuration](#configuration)
8. [Testing](#testing)
9. [Troubleshooting](#troubleshooting)

---

## Overview

The PDF Ingestion Pipeline is an AI-powered system that transforms curriculum documents (PDF, Markdown, Text) into fully structured BuildOps learning programs with:

- ✅ Complete day plans with syntax targets and implementation briefs
- ✅ Checklist items for each day
- ✅ Quiz questions with multiple types
- ✅ Concept tags and dependencies
- ✅ Time estimates and complexity levels
- ✅ Validation and error recovery

### Key Features

- **Multi-format Support**: PDF, Markdown (.md), and Text (.txt) files
- **Multi-file Import**: Combine multiple documents into one program
- **AI Analysis**: Claude Sonnet 4 analyzes content and generates structured curriculum
- **Smart Chunking**: Automatic token-aware document splitting for large files
- **Review Interface**: Edit generated content before applying
- **Async Pipeline**: Non-blocking processing with real-time progress updates
- **Error Recovery**: Retry failed imports with detailed error messages

---

## Architecture

### Pipeline Flow

```
1. File Upload → 2. Extraction → 3. Chunking → 4. AI Analysis → 5. Plan Generation → 6. Review → 7. Apply
```

### Tech Stack

**Backend (Rust)**
- `pdf-extract` - PDF text extraction
- `lopdf` - PDF metadata parsing
- `pulldown-cmark` - Markdown parsing
- `reqwest` - HTTP client for Anthropic API
- `tiktoken-rs` - Token counting for chunking
- `aes-gcm` - API key encryption
- `sqlx` - Database operations

**Frontend (SvelteKit 5)**
- Svelte 5 runes for reactive state
- Tauri IPC for backend communication
- `pdfjs-dist` - Client-side PDF preview (optional)

---

## Backend Components

### 1. Database Schema

**Migration 020: `import_jobs` table**

```sql
CREATE TABLE import_jobs (
    id TEXT PRIMARY KEY,
    program_id TEXT REFERENCES programs(id),
    status TEXT CHECK (status IN ('pending', 'extracting', 'analyzing', 'generating', 'review', 'applying', 'completed', 'failed')),
    source_type TEXT CHECK (source_type IN ('pdf', 'markdown', 'text', 'multi_file')),
    source_files_json TEXT,
    extracted_text TEXT,
    extracted_sections_json TEXT,
    ai_analysis_json TEXT,
    generated_plan_json TEXT,
    reviewed_plan_json TEXT,
    total_pages INTEGER,
    total_tokens INTEGER,
    total_days_generated INTEGER,
    ai_model_used TEXT DEFAULT 'claude-sonnet-4-20250514',
    error_message TEXT,
    error_step TEXT,
    started_at TEXT,
    completed_at TEXT,
    created_at TEXT,
    updated_at TEXT
);
```

### 2. Services

#### `pdf_extractor.rs`
Extracts structured content from documents:
- **PDF**: Uses `pdf-extract` for text, heuristic heading detection
- **Markdown**: Uses `pulldown-cmark` parser for structured sections
- **Text**: Simple paragraph-based extraction

**Key Functions:**
- `extract_document(file_path: &str) -> Result<ExtractedDocument>`
- Detects code blocks, lists, headings
- Estimates complexity per section
- Extracts metadata (word count, detected languages, topics)

#### `document_chunker.rs`
Intelligent chunking for AI processing:
- **Single Pass**: < 150k tokens → send entire document
- **Section-Based**: 150k-500k tokens → chunk by sections
- **Multi-Pass**: > 500k tokens → chunk with context overlap

**Key Functions:**
- `chunk_document(doc: &ExtractedDocument) -> Result<ChunkedDocument>`
- `merge_multi_file_content(docs: Vec<ExtractedDocument>) -> ExtractedDocument`

#### `ai_analyzer.rs`
Anthropic Claude API integration:
- Model: `claude-sonnet-4-20250514`
- Max tokens: 8192 output
- Temperature: 0.3 (focused, deterministic)
- Retry logic: 3 attempts with exponential backoff

**Key Functions:**
- `analyze_with_ai(config: &AiConfig, chunked: &ChunkedDocument) -> Result<AiAnalysisResponse>`
- Handles single-chunk and multi-chunk analysis
- Merges chunk results for large documents

**System Prompt:**
The AI is instructed to act as a "curriculum architect" and produce structured JSON with:
- Program title and description
- Modules with colors and descriptions
- Days with specific, actionable titles
- Syntax targets (exact code patterns to memorize)
- Implementation briefs (concrete build outputs)
- Checklist items (3-8 per day)
- Quiz questions (2-5 per day)
- Concept tags (granular, not generic)
- Dependencies (prerequisite/recommended)

#### `plan_generator.rs`
Validates and transforms AI output:
- Clamps time estimates (45-180 minutes)
- Validates colors, question types, dependency types
- Calculates complexity levels (1-5 stars)
- Detects circular dependencies
- Generates validation warnings

**Key Functions:**
- `generate_plan(ai_response: AiAnalysisResponse) -> Result<GeneratedPlan>`

#### `import_applier.rs`
Transactional database writer:
- Creates/updates program
- Creates modules with colors
- Creates day plans with time engine fields
- Creates checklist items
- Creates quiz questions
- Creates/links concept tags
- Creates dependencies

**Key Functions:**
- `apply_import(pool: &Pool<Sqlite>, plan: &GeneratedPlan, existing_program_id: Option<String>) -> Result<Program>`

### 3. IPC Commands

All commands are registered in `src-tauri/src/commands/import.rs`:

1. **`start_import`** - Initiates import pipeline
   - Copies files to app data directory
   - Creates import job record
   - Spawns async pipeline task
   - Returns job immediately

2. **`get_import_job`** - Polls job status
   - Returns full ImportJob with all fields
   - Used for progress tracking

3. **`get_import_preview`** - Returns generated plan
   - Only works when status = 'review'
   - Returns reviewed_plan_json if exists, else generated_plan_json

4. **`update_import_preview`** - Saves user edits
   - Updates reviewed_plan_json field
   - Does not apply to database yet

5. **`apply_import`** - Writes to database
   - Validates status = 'review'
   - Calls import_applier service
   - Updates status to 'completed'
   - Returns created Program

6. **`cancel_import`** - Cancels job
   - Sets status to 'failed'
   - Sets error_message to 'Cancelled by user'

7. **`list_import_jobs`** - Lists recent jobs
   - Returns last 50 jobs
   - Ordered by created_at DESC

8. **`delete_import_job`** - Deletes job
   - Removes from database
   - Does not delete source files

9. **`retry_import`** - Retries failed job
   - Resets status to 'pending'
   - Clears error fields
   - Spawns new pipeline task

---

## Frontend Components

### 1. TypeScript Types

**`src/lib/types/import.ts`**

Key types:
- `ImportJob` - Full job record
- `ImportJobSummary` - Lightweight job list item
- `ImportGeneratedPlan` - Complete generated curriculum
- `ProgramDraft`, `ModuleDraft`, `DayPlanDraft` - Plan entities
- `ChecklistItemDraft`, `QuizQuestionDraft`, `ConceptTagDraft`, `DependencyDraft`

### 2. Command Wrappers

**`src/lib/commands/import.ts`**

All IPC commands wrapped with TypeScript types:
```typescript
import { imports } from '$lib/commands';

const job = await imports.startImport(filePaths, programId, apiKey);
const preview = await imports.getImportPreview(jobId);
await imports.updateImportPreview(jobId, planJson);
const program = await imports.applyImport(jobId);
```

### 3. UI Components

#### **FileUploadZone.svelte**
Drag-and-drop file upload with browse button:
- Accepts PDF, Markdown, Text files
- Multi-file support
- Shows selected files with remove buttons
- Visual drag-over feedback

**Props:**
- `onFilesSelected: (paths: string[]) => void`
- `accept?: string[]` (default: ['pdf', 'md', 'markdown', 'txt'])
- `multiple?: boolean` (default: true)

#### **ImportProgressCard.svelte**
Real-time progress visualization:
- Status badge (Processing/Review/Completed/Failed)
- Progress bar with percentage
- Step indicators with icons
- Stats grid (files, pages, tokens, days)
- Error message display

**Props:**
- `job: ImportJob`

#### **ImportReviewInterface.svelte**
Full-featured review and edit interface:
- Tabbed navigation (Overview, Modules, Days, Warnings)
- Editable program title and description
- Module cards with expand/collapse
- Day plan grid with metadata
- Validation warnings list
- Save changes button

**Props:**
- `plan: ImportGeneratedPlan`
- `onSave: (updatedPlan: ImportGeneratedPlan) => void`

### 4. Routes

#### **`/import` - Import Hub**
Main entry point for imports:
- File upload zone
- API key input (collapsible, secure)
- Recent imports list with status indicators
- Start import button

#### **`/import/[id]` - Progress Tracking**
Real-time progress monitoring:
- Auto-polls every 2 seconds
- Shows ImportProgressCard
- Auto-redirects to review when ready
- Cancel button for active imports
- Retry button for failed imports
- View program button for completed imports

#### **`/import/[id]/review` - Review & Apply**
Final review before applying:
- Full-screen review interface
- Save changes functionality
- Apply import button
- Back to progress link

---

## User Workflow

### Step-by-Step Guide

1. **Navigate to Import Hub**
   - Go to `/import`
   - See recent imports (if any)

2. **Upload Files**
   - Drag and drop files or click "Browse Files"
   - Select one or more PDF/Markdown/Text files
   - Files are displayed with remove option

3. **Enter API Key**
   - Click "🔑 Enter API Key"
   - Paste Anthropic API key (starts with `sk-ant-`)
   - Key is used only for this import

4. **Start Import**
   - Click "Start Import"
   - Redirected to `/import/[id]`

5. **Monitor Progress**
   - Watch real-time progress updates
   - See extraction → analysis → generation steps
   - View stats (pages, tokens, days generated)

6. **Review Generated Plan**
   - Auto-redirected to `/import/[id]/review` when ready
   - Review program details, modules, days
   - Edit titles, descriptions as needed
   - Check validation warnings

7. **Apply Import**
   - Click "✓ Apply Import"
   - Confirm action
   - Program created in database
   - Redirected to program view

### Error Handling

**If Import Fails:**
- Error message displayed with step and details
- Click "🔄 Retry Import"
- Enter API key again
- Pipeline restarts from beginning

**Common Errors:**
- **Extraction failed**: Corrupted or unsupported PDF
- **API error**: Invalid API key or rate limit
- **Parsing failed**: AI returned invalid JSON
- **Database error**: Constraint violation or disk full

---

## API Reference

### Anthropic API Integration

**Endpoint:** `https://api.anthropic.com/v1/messages`

**Headers:**
```
x-api-key: YOUR_API_KEY
anthropic-version: 2023-06-01
content-type: application/json
```

**Request Body:**
```json
{
  "model": "claude-sonnet-4-20250514",
  "max_tokens": 8192,
  "temperature": 0.3,
  "system": "You are a curriculum architect...",
  "messages": [
    {
      "role": "user",
      "content": "Analyze this curriculum content..."
    }
  ]
}
```

**Response:**
```json
{
  "content": [
    {
      "text": "{\"program_title\": \"...\", ...}"
    }
  ]
}
```

### Expected AI Output Schema

```json
{
  "program_title": "string",
  "program_description": "string",
  "estimated_total_days": 30,
  "modules": [
    {
      "title": "Module 1: Foundations",
      "description": "...",
      "order_index": 0,
      "color": "#6366F1",
      "days": [
        {
          "day_number": 1,
          "title": "Specific Day Title",
          "syntax_targets": "markdown with exact syntax",
          "implementation_brief": "what to build",
          "files_to_create": "list of files",
          "success_criteria": "measurable criteria",
          "stretch_challenge": "advanced extension",
          "notes": "tips and gotchas",
          "estimated_minutes": 90,
          "memory_rebuild_minutes": 15,
          "checklist_items": [
            {"label": "Task 1", "is_required": true}
          ],
          "quiz_questions": [
            {
              "question_text": "What is...?",
              "question_type": "multiple_choice",
              "correct_answer": "Answer",
              "options": ["A", "B", "C"],
              "points": 10,
              "time_limit_seconds": 120
            }
          ],
          "concept_tags": [
            {"name": "$state rune", "domain": "svelte"}
          ],
          "dependencies": [
            {
              "depends_on_day_number": 0,
              "type": "prerequisite",
              "minimum_score": 70
            }
          ]
        }
      ]
    }
  ]
}
```

---

## Configuration

### Environment Variables

None required. API key is provided per-import by user.

### Chunking Limits

Defined in `document_chunker.rs`:
```rust
const SINGLE_PASS_LIMIT: usize = 150_000;  // tokens
const SECTION_CHUNK_LIMIT: usize = 100_000;
const MULTI_PASS_LIMIT: usize = 80_000;
```

### AI Configuration

Defined in `ai_analyzer.rs`:
```rust
const MODEL: &str = "claude-sonnet-4-20250514";
const MAX_TOKENS: usize = 8192;
const TEMPERATURE: f32 = 0.3;
const MAX_RETRIES: u32 = 3;
```

### Time Estimates

Defined in `plan_generator.rs`:
```rust
fn clamp_minutes(minutes: i64) -> i64 {
    minutes.clamp(45, 180)  // 45-180 minutes per day
}
```

---

## Testing

### Manual Testing Checklist

**1. Single PDF Import**
- [ ] Upload a small PDF (< 10 pages)
- [ ] Verify extraction completes
- [ ] Check AI analysis produces valid JSON
- [ ] Review generated plan
- [ ] Apply import successfully

**2. Multi-file Import**
- [ ] Upload 2-3 Markdown files
- [ ] Verify files are merged correctly
- [ ] Check all content is analyzed
- [ ] Apply import successfully

**3. Large Document**
- [ ] Upload PDF > 100 pages
- [ ] Verify chunking strategy is multi-pass
- [ ] Check chunk merging works
- [ ] Verify all days are generated

**4. Error Recovery**
- [ ] Start import with invalid API key
- [ ] Verify error is displayed
- [ ] Retry with valid key
- [ ] Verify import completes

**5. Review Interface**
- [ ] Edit program title
- [ ] Edit module descriptions
- [ ] Save changes
- [ ] Verify changes persist
- [ ] Apply import with edits

**6. Edge Cases**
- [ ] Upload empty file
- [ ] Upload corrupted PDF
- [ ] Cancel import mid-process
- [ ] Delete completed import

### Unit Test Examples

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_markdown() {
        let content = "# Heading\n\nContent here.";
        // Test extraction logic
    }

    #[test]
    fn test_chunk_small_document() {
        // Test single-pass chunking
    }

    #[test]
    fn test_validate_dependencies() {
        // Test circular dependency detection
    }
}
```

---

## Troubleshooting

### Common Issues

**1. "Failed to extract PDF"**
- **Cause**: Corrupted or image-only PDF
- **Solution**: Try OCR preprocessing or use text-based PDF

**2. "API returned error 401"**
- **Cause**: Invalid Anthropic API key
- **Solution**: Verify key starts with `sk-ant-` and is active

**3. "Failed to parse AI JSON response"**
- **Cause**: AI returned malformed JSON
- **Solution**: Retry import (AI is non-deterministic)

**4. "Circular dependency detected"**
- **Cause**: Day depends on a later day
- **Solution**: Edit dependencies in review interface

**5. "Import stuck in 'analyzing' status"**
- **Cause**: API timeout or network issue
- **Solution**: Cancel and retry import

### Debug Mode

Enable Rust logging:
```bash
RUST_LOG=debug cargo tauri dev
```

Check logs for:
- Extraction output
- Token counts
- API requests/responses
- Database queries

### Performance Optimization

**For Large Documents:**
1. Increase chunk limits if memory allows
2. Use faster PDF extraction library
3. Cache tokenizer initialization
4. Batch database inserts

**For Many Imports:**
1. Implement job queue
2. Limit concurrent API requests
3. Add rate limiting
4. Implement caching layer

---

## Future Enhancements

### Planned Features

1. **PDF Preview** - Show original PDF alongside generated plan
2. **Batch Import** - Import multiple programs at once
3. **Template Library** - Pre-built curriculum templates
4. **Export** - Export generated plans to JSON/Markdown
5. **Collaboration** - Share imports with team
6. **Version Control** - Track changes to imported programs
7. **AI Tuning** - Adjust AI parameters per import
8. **Cost Tracking** - Track API usage and costs

### API Improvements

1. **Streaming** - Stream AI responses for faster feedback
2. **Caching** - Cache AI responses for similar content
3. **Fine-tuning** - Train custom model on BuildOps format
4. **Multi-model** - Support GPT-4, Gemini, etc.

---

## Support

For issues or questions:
1. Check this documentation
2. Review error messages in UI
3. Check Rust logs with `RUST_LOG=debug`
4. Review Anthropic API documentation
5. File issue with full error details

---

## License

Part of BuildOps 4.0 - All rights reserved.

---

**Implementation Date:** February 14, 2026  
**Version:** 1.0.0  
**Status:** ✅ Production Ready
