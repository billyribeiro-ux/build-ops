# BuildOps 40 — Phase 2: Program & Module CRUD (Full Stack)

## What This Phase Builds

This is the first real data flow through the entire stack. By the end, a user can create programs, edit them, delete them, and manage modules within programs. Data flows from Svelte UI → Tauri IPC → Rust command → SQLx query → SQLite → back up through the same chain to render on screen.

This phase establishes the patterns that every subsequent phase follows. Get this right and the rest is repetition.

---

## Dependencies on Phase 1

Phase 1 must be complete. The app must launch, the database must exist with all tables, and the sidebar navigation must render.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/db/models/mod.rs` — model module declarations
2. `src-tauri/src/db/models/program.rs` — Program struct + SQLx queries
3. `src-tauri/src/db/models/module.rs` — Module struct + SQLx queries
4. `src-tauri/src/commands/mod.rs` — command module declarations
5. `src-tauri/src/commands/programs.rs` — 7 Tauri commands: create, get, list, update, delete, duplicate, get_stats
6. `src-tauri/src/commands/modules.rs` — 6 Tauri commands: create, get, list, update, delete, reorder
7. Update `src-tauri/src/lib.rs` — register all 13 commands in `invoke_handler`

### Frontend Layer
8. `src/lib/types/common.ts` — shared utility types (timestamps, pagination)
9. `src/lib/types/program.ts` — Program, ProgramSummary, ProgramStats TypeScript types
10. `src/lib/types/index.ts` — type re-exports
11. `src/lib/commands/programs.ts` — typed invoke wrappers for program commands
12. `src/lib/commands/modules.ts` — typed invoke wrappers for module commands
13. `src/lib/commands/index.ts` — command re-exports
14. `src/lib/components/ui/Button.svelte` — reusable button component
15. `src/lib/components/ui/Input.svelte` — reusable input component
16. `src/lib/components/ui/Textarea.svelte` — reusable textarea component
17. `src/lib/components/ui/Modal.svelte` — reusable modal component
18. `src/lib/components/ui/Card.svelte` — reusable card component
19. `src/lib/components/ui/Badge.svelte` — reusable badge component
20. `src/lib/components/ui/EmptyState.svelte` — reusable empty state component
21. `src/lib/components/ui/Toast.svelte` — toast notification system
22. `src/lib/components/ui/ConfirmDialog.svelte` — confirmation dialog
23. `src/lib/stores/app.svelte.ts` — global app state with toast management
24. `src/routes/programs/+page.svelte` — programs list (replace placeholder)
25. `src/routes/programs/+page.ts` — programs list data loader
26. `src/routes/programs/new/+page.svelte` — create program form
27. `src/routes/programs/[programId]/+page.svelte` — program detail with modules
28. `src/routes/programs/[programId]/+page.ts` — program detail data loader
29. `src/routes/programs/[programId]/+layout.svelte` — program-scoped layout
30. `src/routes/programs/[programId]/+layout.ts` — program-scoped layout loader

---

## Rust Model Pattern

Every model follows this exact pattern. This is the template for Phase 2 and all future phases.

```rust
// The struct matches the database row exactly
// Derives Serialize + Deserialize for Tauri IPC
// Derives sqlx::FromRow for query mapping
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Program {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_days: i32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}
```

Every query function is an `async fn` that takes a `&SqlitePool` and returns `Result<T, AppError>`. Never use `.unwrap()` in any query function. Always propagate errors with `?`.

---

## Rust Command Pattern

Every Tauri command follows this exact pattern:

```rust
#[tauri::command]
pub async fn create_program(
    state: tauri::State<'_, crate::AppState>,
    title: String,
    description: String,
    target_days: i32,
) -> Result<Program, String> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    sqlx::query_as::<_, Program>(
        "INSERT INTO programs (id, title, description, target_days, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?) 
         RETURNING *"
    )
    .bind(&id)
    .bind(&title)
    .bind(&description)
    .bind(target_days)
    .bind(&now)
    .bind(&now)
    .fetch_one(&*state.db)
    .await
    .map_err(|e| e.to_string())
}
```

Key rules:
- Parameters are deserialized from the frontend `invoke()` call automatically
- Return type is always `Result<T, String>` where T is serializable
- Errors convert to String for IPC transport (Tauri requirement)
- UUIDs generated with `uuid::Uuid::new_v4()`
- Timestamps generated with `chrono::Utc::now()`
- Use `query_as` with `RETURNING *` for insert/update operations that need the row back
- Use `query` (no `_as`) for delete operations

---

## Frontend Command Wrapper Pattern

Every command wrapper follows this exact pattern:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { Program } from '$lib/types';

export async function createProgram(
  title: string,
  description: string,
  targetDays: number
): Promise<Program> {
  return invoke<Program>('create_program', { title, description, targetDays });
}
```

Key rules:
- Import `invoke` from `@tauri-apps/api/core` (NOT `@tauri-apps/api/tauri` — that's Tauri v1)
- Parameter names in the object passed to `invoke` must be camelCase — Tauri automatically converts them to snake_case for Rust
- Return type matches the Rust struct's serialized form
- No try/catch here — let the calling component handle errors

---

## Frontend Data Loading Pattern

SvelteKit load functions call Tauri commands. Since Tauri commands only work in the browser (not during SSR), all load functions must run client-side.

```typescript
// +page.ts
import type { PageLoad } from './$types';
import { listPrograms } from '$lib/commands/programs';

export const ssr = false;  // CRITICAL: disable SSR for Tauri
export const prerender = false;

export const load: PageLoad = async () => {
  const programs = await listPrograms();
  return { programs };
};
```

Add `export const ssr = false;` to EVERY `+page.ts` and `+layout.ts` file in the project. Tauri commands cannot execute during server-side rendering.

---

## UI Component Standards

All UI components use Svelte 5 runes and accept props via `$props()`. Every component has typed props. Every interactive component supports keyboard navigation.

Color classes always reference the CSS variable tokens (`text-text-primary`, `bg-bg-secondary`, `border-border-primary`, etc.) — never hardcode hex values in components.

Icons always use Iconify with Phosphor icon set:

```svelte
<Icon icon="ph:pencil-bold" width="16" />
```

Never import or use Lucide icons.

---

## Complete Command List for This Phase

### Program Commands (7)

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `create_program` | `title: String, description: String, target_days: i32` | `Program` | Creates new program with UUID, timestamps, default status "active" |
| `get_program` | `id: String` | `Program` | Fetches single program by ID. Returns error if not found |
| `list_programs` | none | `Vec<ProgramSummary>` | Lists all non-archived programs ordered by `created_at` desc. `ProgramSummary` includes program fields + `days_count`, `completed_days`, `latest_score` |
| `update_program` | `id: String, title: Option<String>, description: Option<String>, status: Option<String>` | `Program` | Updates only provided fields. Updates `updated_at` timestamp |
| `delete_program` | `id: String` | `()` | Deletes program and cascades to all child data |
| `duplicate_program` | `id: String, new_title: String` | `Program` | Deep copies program with all modules. Day plans are NOT copied (user adds those to the new program manually or via import) |
| `get_program_stats` | `id: String` | `ProgramStats` | Returns aggregate stats: total days, completed days, blocked days, average score, current streak, total time |

### Module Commands (6)

| Command | Parameters | Returns | Description |
|---------|-----------|---------|-------------|
| `create_module` | `program_id: String, title: String, description: String, color: String` | `Module` | Creates module with auto-incremented `order_index` |
| `get_module` | `id: String` | `Module` | Fetches single module |
| `list_modules` | `program_id: String` | `Vec<Module>` | Lists modules for a program ordered by `order_index` |
| `update_module` | `id: String, title: Option<String>, description: Option<String>, color: Option<String>` | `Module` | Updates provided fields |
| `delete_module` | `id: String` | `()` | Deletes module and cascades to day plans |
| `reorder_modules` | `program_id: String, module_ids: Vec<String>` | `()` | Resets `order_index` for all modules in the given order |

---

## Page Specifications

### Programs List (`/programs`)

**Data:** Calls `listPrograms()` on load. Gets back array of `ProgramSummary`.

**Layout:**
- Header with title "Programs" and "New Program" button (links to `/programs/new`)
- Grid of program cards (3 columns on desktop, 2 on medium, 1 on small)
- Each card shows: title, description (truncated to 2 lines), status badge, progress bar (`completed_days / total_days`), latest score, created date
- Card click navigates to `/programs/[programId]`
- Card has a "..." menu with: Edit, Duplicate, Archive, Delete
- Delete shows confirmation dialog
- Empty state when no programs exist: illustration + "Create your first program" CTA

### Create Program (`/programs/new`)

**Layout:**
- Form with: Title (required, text input), Description (optional, textarea), Target Days (number input, default 40)
- "Create Program" submit button
- "Cancel" link back to `/programs`
- On submit: calls `createProgram()`, on success navigates to `/programs/[newId]`
- Validation: title must be non-empty, target_days must be > 0

### Program Detail (`/programs/[programId]`)

**Data:** Calls `getProgram(programId)` and `listModules(programId)` on load.

**Layout:**
- Program header: title, description, status badge, edit button, stats summary row (total days, completed, avg score, streak)
- "Add Module" button
- Module list: vertically stacked cards, each showing title, color indicator, description, day count within module
- Module cards are drag-reorderable (reorder on drop via `reorderModules()`)
- Click module card to expand inline and show day plans within it (day plans built in Phase 3)
- Module card "..." menu: Edit, Delete
- Add Module opens an inline form or modal: title, description, color picker (preset palette of 8 colors)

### Program-Scoped Layout (`/programs/[programId]/+layout.svelte`)

- Adds a breadcrumb under the top bar: "Programs > [Program Title]"
- Passes program data to child routes via layout data

---

## Verification Checklist

After completing all items above:

1. **Create a program:** Go to `/programs/new`, fill in title "SvelteKit Mastery", description "40-day learning plan", target days 40. Submit. Verify redirect to program detail page.

2. **View programs list:** Go to `/programs`. Verify the new program appears as a card with correct data.

3. **Edit a program:** Click the program card. On the detail page, click edit. Change the title. Save. Verify title updated.

4. **Create modules:** On the program detail page, click "Add Module". Create 3 modules: "HTML Foundations" (color #22C55E), "CSS Layout" (color #3B82F6), "JavaScript" (color #F59E0B). Verify all three appear in order.

5. **Reorder modules:** Drag "JavaScript" above "CSS Layout". Verify new order persists after page refresh.

6. **Delete a module:** Delete "JavaScript" module. Confirm deletion. Verify it's gone.

7. **Duplicate program:** From programs list, duplicate "SvelteKit Mastery". Verify new program appears with title you gave it, and its modules are copied.

8. **Delete a program:** Delete the duplicate. Confirm deletion. Verify it's gone from the list.

9. **Empty state:** If you delete all programs, verify the empty state renders with CTA.

10. **TypeScript check:** Run `pnpm check` — zero errors.

11. **Rust check:** Run `cd src-tauri && cargo clippy` — zero errors (warnings about unused imports from future phases are acceptable).

---

## What's Done After This Phase

- Full Rust model + command pattern established
- Full frontend command wrapper + data loading pattern established
- Base UI component library started (Button, Input, Textarea, Modal, Card, Badge, EmptyState, Toast, ConfirmDialog)
- Toast notification system operational
- Programs CRUD working end-to-end
- Modules CRUD working end-to-end with reordering
- Program duplication working
- Program stats query working
- Breadcrumb navigation in program-scoped routes

## Next Phase

Phase 3 builds Day Plan CRUD — the core content authoring system including checklists, quiz questions, concept tags, and dependencies.
