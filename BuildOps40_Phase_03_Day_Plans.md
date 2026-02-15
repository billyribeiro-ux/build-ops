# BuildOps 40 — Phase 3: Day Plan System (Full Stack)

## What This Phase Builds

The day plan is the atomic unit of BuildOps 40 — everything revolves around it. This phase builds the complete authoring system: creating day plans with all content fields, managing checklists, quiz questions, concept tags, and dependencies. By the end, a user can fully author a curriculum within the app.

---

## Dependencies on Phase 2

Phase 2 must be complete. Programs and modules must be fully operational. The UI component library must include Button, Input, Textarea, Modal, Card, Badge, EmptyState, Toast, and ConfirmDialog.

---

## What Gets Built

### Rust Layer
1. `src-tauri/src/db/models/day_plan.rs` — DayPlan, DayPlanFull, DayPlanSummary structs + queries
2. `src-tauri/src/db/models/checklist.rs` — ChecklistItem, AttemptChecklist structs + queries
3. `src-tauri/src/db/models/quiz.rs` — QuizQuestion struct + queries
4. `src-tauri/src/db/models/concept_tag.rs` — ConceptTag struct + queries + tag assignment queries
5. `src-tauri/src/db/models/dependency.rs` — DayDependency, DependencyStatus structs + queries
6. `src-tauri/src/commands/days.rs` — 19 Tauri commands covering day plans, checklists, quizzes, tags, dependencies
7. Update `src-tauri/src/lib.rs` — register all 19 new commands

### Frontend Layer
8. `src/lib/types/program.ts` — add DayPlan, DayPlanFull, DayPlanSummary, ChecklistItem, QuizQuestion, ConceptTag, DayDependency, DependencyStatus types
9. `src/lib/commands/days.ts` — typed invoke wrappers for all 19 commands
10. `src/lib/components/ui/Select.svelte` — dropdown select component
11. `src/lib/components/ui/Checkbox.svelte` — checkbox component
12. `src/lib/components/ui/Tabs.svelte` — tabbed interface component
13. `src/lib/components/ui/Tooltip.svelte` — tooltip component
14. `src/lib/components/program/DayPlanCard.svelte` — compact card for day plan in module list
15. `src/lib/components/program/DayPlanEditor.svelte` — full authoring form for day plans
16. `src/lib/components/program/ModuleEditor.svelte` — module with expandable day plan list
17. `src/lib/components/program/TemplateSelector.svelte` — day plan template selector
18. `src/lib/components/editor/MarkdownEditor.svelte` — markdown textarea with preview toggle (CodeMirror deferred to Phase 4, use a styled textarea with `marked` preview for now)
19. `src/lib/components/editor/MarkdownPreview.svelte` — renders markdown to sanitized HTML
20. `src/routes/programs/[programId]/+page.svelte` — update to show modules with day plans inside them
21. `src/routes/programs/[programId]/days/new/+page.svelte` — create day plan page
22. `src/routes/programs/[programId]/days/[dayId]/+page.svelte` — day mission view (read-only view of the plan)
23. `src/routes/programs/[programId]/days/[dayId]/+page.ts` — day mission data loader

---

## Day Plan Commands (19 total)

### Core Day Plan CRUD (7)
| Command | Description |
|---------|-------------|
| `create_day_plan` | Takes `CreateDayPlanInput` struct with all content fields. Auto-assigns next `day_number` within the program. Sets version to 1, status to 'draft'. |
| `get_day_plan` | Returns `DayPlanFull` — the day plan with its checklist items, quiz questions, concept tags, and dependencies all joined in one response. This is a single call that returns everything needed to render the day mission screen. |
| `list_day_plans` | Returns `Vec<DayPlanSummary>` for a program, ordered by day_number. Summary includes: id, title, day_number, module_id, module_title, module_color, status, estimated_minutes, checklist_count, quiz_count, tag_count, best_score (from day_attempts), attempt_count. |
| `list_day_plans_by_module` | Same as above but filtered to a single module. |
| `update_day_plan` | Updates provided fields. If day plan status is 'published' and content fields change, increments version number and logs the change. Always updates `updated_at`. |
| `delete_day_plan` | Deletes day plan and cascades. Resequences remaining day numbers within the program to close gaps. |
| `reorder_day_plans` | Accepts ordered list of day plan IDs, resets `day_number` for each sequentially starting from 1. |
| `duplicate_day_plan` | Deep copies a day plan including checklists and quiz questions. Assigns next available day_number. Does NOT copy attempts or evidence. |

### Checklist Management (4)
| Command | Description |
|---------|-------------|
| `add_checklist_item` | Adds item to a day plan. Auto-assigns next `order_index`. |
| `update_checklist_item` | Updates label and/or is_required flag. |
| `delete_checklist_item` | Removes item. Resequences remaining order indices. |
| `reorder_checklist_items` | Resets order indices based on provided ID order. |

### Quiz Management (3)
| Command | Description |
|---------|-------------|
| `add_quiz_question` | Takes `CreateQuizQuestionInput` with question_text, question_type, correct_answer, options (JSON array for multiple choice), points, time_limit_seconds. Auto-assigns order_index. |
| `update_quiz_question` | Updates provided fields. |
| `delete_quiz_question` | Removes question. Resequences order indices. |

### Concept Tags (4)
| Command | Description |
|---------|-------------|
| `create_concept_tag` | Creates a new tag with name, domain, color. Name must be unique (case-insensitive). |
| `list_concept_tags` | Lists all concept tags ordered by domain then name. |
| `add_tag_to_day` | Creates association between day_plan and tag. Idempotent — if already exists, no-op. |
| `remove_tag_from_day` | Removes association. |

### Dependencies (3)
| Command | Description |
|---------|-------------|
| `add_dependency` | Creates dependency: day_plan_id depends on depends_on_id with type and minimum_score. Validates no circular dependencies (checks full dependency chain). |
| `remove_dependency` | Removes a dependency. |
| `check_dependencies` | Returns `Vec<DependencyStatus>` for a day plan. Each entry includes: the dependency definition, the depended-on day plan title/number, whether it's met (has an attempt with score >= minimum), the current best score for that dependency. |

---

## Day Plan Editor Page (`/programs/[programId]/days/new`)

This is the full authoring interface. It must handle both creating new day plans and editing existing ones (same component, different mode).

**Layout: Single scrollable form with sections**

1. **Header section**
   - Title (text input, required)
   - Module selector (dropdown of existing modules in this program)
   - Day number (auto-assigned, editable for manual override)
   - Estimated time (number input in minutes, default 60)
   - Memory rebuild time (number input in minutes, default 15)
   - Status toggle: Draft / Published

2. **Content section** — each field is a markdown editor with preview toggle
   - Syntax Targets
   - Implementation Brief
   - Files to Create
   - Success Criteria
   - Stretch Challenge
   - Notes

3. **Checklist section**
   - List of checklist items with inline edit
   - Each item: label text input + required checkbox + delete button + drag handle for reorder
   - "Add Item" button at bottom
   - Minimum 1 item required before publishing

4. **Quiz section**
   - List of quiz questions, each expandable
   - Each question: type selector (short_answer, multiple_choice, code_prompt, reflection), question text (textarea), correct answer (textarea), options list (for multiple_choice only), points (number), time limit (number in seconds)
   - "Add Question" button
   - Minimum 1 question required before publishing

5. **Concept Tags section**
   - Tag selector: searchable dropdown of existing tags + "Create new tag" option
   - Selected tags display as colored badges with remove button
   - Creating a new tag: name, domain (dropdown: HTML, CSS, JavaScript, Svelte 5, SvelteKit, TypeScript, GSAP, Forms, SEO, Other), color

6. **Dependencies section**
   - List of dependencies
   - Each: dropdown selecting another day plan in this program + type (prerequisite/recommended/related) + minimum score (number, default 70)
   - "Add Dependency" button
   - Cannot select self. Circular dependency check runs on save.

7. **Action bar** (sticky at bottom)
   - "Save as Draft" button
   - "Save & Publish" button
   - "Cancel" link

**On save:** Calls `create_day_plan` (or `update_day_plan`), then calls checklist/quiz/tag/dependency commands to sync sub-entities. Shows toast on success, navigates to day mission view.

---

## Day Mission View (`/programs/[programId]/days/[dayId]`)

This is the read-only presentation of a day plan. The working screen (where you actually do the day's work) is built in Phase 4.

**Data:** Calls `get_day_plan(dayId)` which returns `DayPlanFull`.

**Layout:**
1. **Header** — Day number badge, title, module badge (colored), status badge, estimated time, memory rebuild time
2. **Dependency alerts** — If any prerequisite dependencies are unmet, show warning banner with details. "Start Attempt" button is disabled if prerequisites not met.
3. **Content cards** — Each content section (Syntax Targets, Implementation Brief, Files to Create, Success Criteria, Stretch Challenge, Notes) renders as a card with the markdown content. Empty sections are hidden.
4. **Checklist preview** — Read-only list of checklist items with required indicators
5. **Quiz preview** — Shows question count and total points, does not reveal questions (those are shown during the actual attempt)
6. **Concept tags** — Displays as colored badges
7. **Attempt history** — Table showing all past attempts for this day: attempt number, date, total score, status badge. Links to attempt detail (built in Phase 4).
8. **Action buttons**
   - "Start Attempt" — creates new attempt, navigates to working screen (Phase 4)
   - "Edit Day Plan" — navigates to editor in edit mode
   - "Duplicate Day" — duplicates this day plan

---

## Program Detail Updates

The program detail page (`/programs/[programId]`) must be updated to show day plans within modules:

- Each module card becomes expandable
- When expanded, shows a list of `DayPlanCard` components inside it
- Each card shows: day number, title, status badge, estimated time, best score (if attempted), attempt count
- Click a day plan card to navigate to its mission view
- "Add Day" button inside each module to create a day plan pre-assigned to that module
- Day plans within a module can be drag-reordered
- An "Add Day" button also exists at the program level (above modules) for adding days to any module

---

## Verification Checklist

1. **Create a day plan:** Navigate to a program, click "Add Day" inside a module. Fill in title "HTML Boilerplate", syntax targets, implementation brief. Add 3 checklist items. Add 2 quiz questions (one short_answer, one multiple_choice). Add 2 concept tags ("doctype", "semantic HTML"). Save as draft. Verify redirect to mission view.

2. **View mission:** Verify all content renders correctly as markdown. Verify checklist items show. Verify tags display. Verify quiz shows question count.

3. **Publish day plan:** Edit the day plan, change status to Published. Save. Verify status badge updates.

4. **Edit a published plan:** Edit a published day plan's syntax targets. Save. Verify version incremented to 2.

5. **Create dependencies:** Create Day 2 plan. Add dependency on Day 1 with type "prerequisite" and minimum score 70. Verify dependency shows on Day 2 mission view. Verify "Start Attempt" button shows dependency warning (since Day 1 has no passing attempt yet).

6. **Circular dependency prevention:** Try to add Day 1 depends on Day 2 (while Day 2 already depends on Day 1). Verify error is returned and dependency is not created.

7. **Reorder days:** Create Day 3. Drag it above Day 2. Verify day numbers update to reflect new order.

8. **Delete a day plan:** Delete Day 3. Verify it's gone and Day 1 and Day 2 numbers are sequential.

9. **Duplicate a day plan:** Duplicate Day 1. Verify new day plan has same content, checklists, quiz questions, tags, but different ID and next day number.

10. **Program overview:** Verify modules on the program page show correct day plan counts and cards.

11. **TypeScript check:** `pnpm check` — zero errors.

---

## What's Done After This Phase

- Complete day plan authoring system
- Markdown editor with preview for all content fields
- Checklist builder with reordering
- Quiz question builder with multiple types
- Concept tag system with creation and assignment
- Dependency system with circular dependency prevention
- Day mission view showing all plan content
- Day plan cards within modules on program overview
- Reordering and duplication for day plans

## Next Phase

Phase 4 builds the Day Attempt working screen — where actual learning happens. This is the most complex screen in the app with the code editor, autosave, timer, and score submission.
