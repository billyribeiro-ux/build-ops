# BuildOps 40 — Complete Product Specification & Build Prompt

## For: Windsurf IDE + Claude AI — End-to-End Implementation

### Author: Billy Ribeiro | Principal Engineer | Explosive Swings / Revolution Trading Pros

### Target: macOS Native Desktop Application (Direct Distribution, NOT App Store)

---

## CRITICAL INSTRUCTIONS FOR THE AI AGENT

You are building a production-grade macOS desktop application called **BuildOps 40** — a personal engineering learning operating system. This is NOT a prototype, NOT an MVP sketch, NOT a simplified example. Every file you generate must be complete, compilable, type-safe, and production-ready.

### Non-Negotiable Engineering Standards

- **Apple Principal Engineer ICT Level 7+ quality** — every component, every function, every type.
- **Microsoft enterprise-grade standards** — zero warnings, zero `any` types, zero shortcuts.
- **TypeScript strict mode** in `tsconfig.json` — `strict: true`, `noUncheckedIndexedAccess: true`, `exactOptionalPropertyTypes: true`.
- **Rust** — `#![deny(clippy::all, clippy::pedantic)]`, zero `unwrap()` in production paths, proper error propagation with `thiserror` + `anyhow`.
- **Every file must be complete** — no `// TODO`, no `// add more here`, no placeholder implementations. If a function exists, it works fully.
- **10-year longevity architecture** — no hacky workarounds, no tech debt shortcuts.
- Build the ENTIRE application. Do not stop partway. Do not ask "should I continue?" — just build it all.

---

## 1. APPLICATION OVERVIEW

### What BuildOps 40 Is

A structured learning execution desktop application that transforms curriculum plans into daily execution sprints with measurable quality gates, proof-of-work evidence, progression analytics, spaced repetition, and a full-text-searchable knowledge base.

### What It Is NOT

- Not a simple to-do list or checklist app
- Not a note-taking app with a progress bar
- Not hardcoded to 40 days — fully dynamic program length
- Not dependent on any cloud service — 100% local-first, offline-capable

### Core Philosophy

The app tracks **engineering quality + reasoning ability + rebuild-from-memory capability**, not just completion. It answers: "Did I build it correctly, can I explain it, and can I rebuild it from memory?"

---

## 2. TECHNOLOGY STACK — EXACT VERSIONS AND DEPENDENCIES

### Shell — Tauri v2

```toml
# src-tauri/Cargo.toml
[package]
name = "buildops40"
version = "0.1.0"
edition = "2021"
rust-version = "1.77"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
tauri-plugin-notification = "2"
tauri-plugin-os = "2"
tauri-plugin-clipboard-manager = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono", "uuid"] }
tokio = { version = "1", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "2"
anyhow = "1"
argon2 = "0.5"
jsonwebtoken = "9"
genpdf = "0.3"
pulldown-cmark = "0.12"
tantivy = "0.22"
notify = "7"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

```toml
# src-tauri/tauri.conf.json — key settings
{
  "productName": "BuildOps 40",
  "version": "0.1.0",
  "identifier": "com.billyribeiro.buildops40",
  "build": {
    "frontendDist": "../build",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "app": {
    "macOSPrivateApi": true,
    "windows": [
      {
        "title": "BuildOps 40",
        "width": 1440,
        "height": 900,
        "minWidth": 1024,
        "minHeight": 700,
        "resizable": true,
        "decorations": true,
        "transparent": false,
        "titleBarStyle": "Overlay"
      }
    ],
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; img-src 'self' asset: https://asset.localhost"
    }
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "app"],
    "macOS": {
      "minimumSystemVersion": "13.0",
      "signingIdentity": null,
      "entitlements": null
    },
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### Frontend — SvelteKit 5 + Svelte 5

```json
// package.json
{
  "name": "buildops40",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite dev",
    "build": "vite build",
    "preview": "vite preview",
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch",
    "lint": "eslint .",
    "format": "prettier --write .",
    "tauri": "tauri"
  },
  "devDependencies": {
    "@sveltejs/adapter-static": "^3.0.0",
    "@sveltejs/kit": "^2.10.0",
    "@sveltejs/vite-plugin-svelte": "^4.0.0",
    "@tailwindcss/typography": "^0.5.15",
    "@tailwindcss/forms": "^0.5.9",
    "@types/node": "^22.0.0",
    "autoprefixer": "^10.4.20",
    "eslint": "^9.0.0",
    "postcss": "^8.4.49",
    "prettier": "^3.4.0",
    "prettier-plugin-svelte": "^3.3.0",
    "svelte": "^5.10.0",
    "svelte-check": "^4.1.0",
    "tailwindcss": "^3.4.17",
    "tslib": "^2.8.0",
    "typescript": "^5.7.0",
    "vite": "^6.0.0"
  },
  "dependencies": {
    "@iconify/svelte": "^4.2.0",
    "@tauri-apps/api": "^2.2.0",
    "@tauri-apps/plugin-dialog": "^2.2.0",
    "@tauri-apps/plugin-fs": "^2.2.0",
    "@tauri-apps/plugin-notification": "^2.2.0",
    "@tauri-apps/plugin-os": "^2.2.0",
    "@tauri-apps/plugin-shell": "^2.2.0",
    "@tauri-apps/plugin-clipboard-manager": "^2.2.0",
    "codemirror": "^6.0.1",
    "@codemirror/lang-javascript": "^6.2.0",
    "@codemirror/lang-html": "^6.4.0",
    "@codemirror/lang-css": "^6.3.0",
    "@codemirror/lang-json": "^6.0.1",
    "@codemirror/lang-markdown": "^6.3.0",
    "@codemirror/lang-rust": "^6.0.1",
    "@codemirror/lang-python": "^6.1.0",
    "@codemirror/theme-one-dark": "^6.1.0",
    "@codemirror/autocomplete": "^6.18.0",
    "@codemirror/lint": "^6.8.0",
    "codemirror-svelte": "^0.4.0",
    "echarts": "^5.5.0",
    "svelte-echarts": "^1.0.0",
    "marked": "^15.0.0",
    "dompurify": "^3.2.0",
    "date-fns": "^4.1.0",
    "fuse.js": "^7.0.0",
    "bits-ui": "^1.0.0",
    "clsx": "^2.1.0",
    "tailwind-merge": "^2.6.0"
  }
}
```

### CRITICAL: Svelte 5 Patterns ONLY

**NEVER use these deprecated patterns:**

```svelte
<!-- FORBIDDEN — Svelte 4 patterns -->
export let prop;              // Use $props() instead
$: derived = value * 2;      // Use $derived() instead
$: { sideEffect(); }         // Use $effect() instead
<slot />                      // Use {@render children()} with snippets instead
on:click={handler}            // Use onclick={handler} instead
createEventDispatcher()       // Use callback props instead
$$props, $$restProps           // Use $props() with rest syntax instead
```

**ALWAYS use these Svelte 5 rune patterns:**

```svelte
<script lang="ts">
  // Props
  let { title, items, onselect }: {
    title: string;
    items: string[];
    onselect: (item: string) => void;
  } = $props();

  // Reactive state
  let count = $state(0);
  let user = $state<User | null>(null);

  // Derived values
  let doubled = $derived(count * 2);
  let fullName = $derived(`${user?.first} ${user?.last}`);

  // Complex derived
  let filtered = $derived.by(() => {
    return items.filter(i => i.includes(searchTerm));
  });

  // Side effects
  $effect(() => {
    console.log(`Count changed to ${count}`);
    return () => { /* cleanup */ };
  });

  // Snippets replace slots
  // Parent passes snippet, child renders with {@render}
</script>
```

### Icons — Phosphor via Iconify ONLY

```svelte
<!-- CORRECT -->
<script lang="ts">
  import Icon from '@iconify/svelte';
</script>
<Icon icon="ph:house-bold" width="24" />
<Icon icon="ph:chart-line-bold" width="24" />
<Icon icon="ph:code-bold" width="24" />
<Icon icon="ph:check-circle-bold" width="24" />

<!-- FORBIDDEN — never use Lucide -->
<!-- import { Home } from 'lucide-svelte' — DO NOT USE -->
```

### Package Manager — pnpm ONLY

```bash
# CORRECT
pnpm install
pnpm add <package>
pnpm dev

# FORBIDDEN
npm install    # NEVER
yarn install   # NEVER
```

### SvelteKit Adapter — Static for Tauri

```typescript
// svelte.config.js
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: 'index.html'
    })
  }
};

export default config;
```

---

## 3. PROJECT STRUCTURE

```
buildops40/
├── src/                                    # SvelteKit frontend
│   ├── app.html
│   ├── app.css                            # Tailwind imports + global styles
│   ├── app.d.ts                           # Global type declarations
│   ├── lib/
│   │   ├── types/
│   │   │   ├── index.ts                   # Re-exports all types
│   │   │   ├── program.ts                 # Program, Module, DayPlan types
│   │   │   ├── attempt.ts                 # DayRun, ScoreCard, Evidence types
│   │   │   ├── quiz.ts                    # Quiz, QuizAttempt types
│   │   │   ├── analytics.ts              # SkillMetric, WeeklyReview types
│   │   │   ├── search.ts                 # Search result types
│   │   │   └── common.ts                 # Shared utility types
│   │   ├── stores/
│   │   │   ├── app.svelte.ts             # Global app state (Svelte 5 runes)
│   │   │   ├── program.svelte.ts         # Active program state
│   │   │   ├── dashboard.svelte.ts       # Dashboard computed state
│   │   │   ├── timer.svelte.ts           # Session timer state
│   │   │   └── search.svelte.ts          # Search state
│   │   ├── commands/
│   │   │   ├── index.ts                   # Re-exports all command modules
│   │   │   ├── programs.ts               # Tauri IPC: program CRUD
│   │   │   ├── modules.ts                # Tauri IPC: module CRUD
│   │   │   ├── days.ts                   # Tauri IPC: day plan CRUD
│   │   │   ├── attempts.ts              # Tauri IPC: day attempt CRUD
│   │   │   ├── quizzes.ts               # Tauri IPC: quiz operations
│   │   │   ├── artifacts.ts             # Tauri IPC: file/evidence management
│   │   │   ├── analytics.ts             # Tauri IPC: scoring, metrics, reviews
│   │   │   ├── search.ts                # Tauri IPC: full-text search
│   │   │   └── export.ts                # Tauri IPC: PDF/JSON/CSV export
│   │   ├── components/
│   │   │   ├── ui/                        # Base UI primitives
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Input.svelte
│   │   │   │   ├── Textarea.svelte
│   │   │   │   ├── Select.svelte
│   │   │   │   ├── Checkbox.svelte
│   │   │   │   ├── Badge.svelte
│   │   │   │   ├── Card.svelte
│   │   │   │   ├── Modal.svelte
│   │   │   │   ├── Dropdown.svelte
│   │   │   │   ├── Tabs.svelte
│   │   │   │   ├── Toast.svelte
│   │   │   │   ├── Tooltip.svelte
│   │   │   │   ├── ProgressRing.svelte
│   │   │   │   ├── ProgressBar.svelte
│   │   │   │   ├── Skeleton.svelte
│   │   │   │   ├── EmptyState.svelte
│   │   │   │   ├── ConfirmDialog.svelte
│   │   │   │   └── FileDropZone.svelte
│   │   │   ├── layout/
│   │   │   │   ├── Sidebar.svelte
│   │   │   │   ├── TopBar.svelte
│   │   │   │   ├── CommandPalette.svelte
│   │   │   │   └── BreadcrumbNav.svelte
│   │   │   ├── dashboard/
│   │   │   │   ├── TodayCard.svelte
│   │   │   │   ├── ProgressRingChart.svelte
│   │   │   │   ├── StreakDisplay.svelte
│   │   │   │   ├── QualityScoreTrend.svelte
│   │   │   │   ├── BlockedAlerts.svelte
│   │   │   │   ├── RecentActivity.svelte
│   │   │   │   └── UpcomingDays.svelte
│   │   │   ├── mission/
│   │   │   │   ├── MissionHeader.svelte
│   │   │   │   ├── SyntaxTargets.svelte
│   │   │   │   ├── ImplementationBrief.svelte
│   │   │   │   ├── FileChecklist.svelte
│   │   │   │   ├── TaskChecklist.svelte
│   │   │   │   ├── ScoreCardForm.svelte
│   │   │   │   ├── MemoryRebuildPanel.svelte
│   │   │   │   ├── SessionTimer.svelte
│   │   │   │   └── DayStatusBadge.svelte
│   │   │   ├── evidence/
│   │   │   │   ├── EvidenceLocker.svelte
│   │   │   │   ├── ArtifactUploader.svelte
│   │   │   │   ├── LinkAttacher.svelte
│   │   │   │   ├── ScreenshotViewer.svelte
│   │   │   │   └── BugLogEntry.svelte
│   │   │   ├── editor/
│   │   │   │   ├── CodeEditor.svelte
│   │   │   │   ├── MarkdownEditor.svelte
│   │   │   │   ├── MarkdownPreview.svelte
│   │   │   │   └── DiffViewer.svelte
│   │   │   ├── quiz/
│   │   │   │   ├── QuizRunner.svelte
│   │   │   │   ├── QuizQuestion.svelte
│   │   │   │   ├── QuizTimer.svelte
│   │   │   │   ├── ReflectionForm.svelte
│   │   │   │   └── QuizResults.svelte
│   │   │   ├── analytics/
│   │   │   │   ├── SkillRadar.svelte
│   │   │   │   ├── WeeklyReviewCard.svelte
│   │   │   │   ├── ConceptHeatmap.svelte
│   │   │   │   ├── BurndownChart.svelte
│   │   │   │   ├── ScoreTrendLine.svelte
│   │   │   │   ├── TimeTracker.svelte
│   │   │   │   └── ForgettingCurveAlert.svelte
│   │   │   ├── program/
│   │   │   │   ├── ProgramManager.svelte
│   │   │   │   ├── ModuleEditor.svelte
│   │   │   │   ├── DayPlanEditor.svelte
│   │   │   │   ├── DayPlanCard.svelte
│   │   │   │   ├── DependencyGraph.svelte
│   │   │   │   ├── RoadmapTimeline.svelte
│   │   │   │   └── TemplateSelector.svelte
│   │   │   └── search/
│   │   │       ├── GlobalSearch.svelte
│   │   │       ├── SearchResults.svelte
│   │   │       └── SearchFilters.svelte
│   │   ├── utils/
│   │   │   ├── cn.ts                      # clsx + tailwind-merge utility
│   │   │   ├── format.ts                 # Date, number, score formatting
│   │   │   ├── scoring.ts                # Score calculation logic
│   │   │   ├── spaced-repetition.ts      # SM-2 algorithm implementation
│   │   │   ├── validators.ts             # Form validation utilities
│   │   │   ├── keyboard.ts               # Keyboard shortcut utilities
│   │   │   ├── debounce.ts               # Debounce/throttle utilities
│   │   │   └── constants.ts              # App-wide constants
│   │   └── config/
│   │       ├── theme.ts                   # Design token definitions
│   │       ├── navigation.ts             # Sidebar nav configuration
│   │       └── scoring.ts                # Scoring weights and thresholds
│   ├── routes/
│   │   ├── +layout.svelte                # Root layout — sidebar + topbar + main area
│   │   ├── +layout.ts                    # Root layout load — initialize app state
│   │   ├── +page.svelte                  # Dashboard (home route)
│   │   ├── +page.ts                      # Dashboard data loader
│   │   ├── programs/
│   │   │   ├── +page.svelte              # All programs list
│   │   │   ├── +page.ts
│   │   │   ├── new/
│   │   │   │   └── +page.svelte          # Create new program
│   │   │   └── [programId]/
│   │   │       ├── +page.svelte          # Program overview — modules + roadmap
│   │   │       ├── +page.ts
│   │   │       ├── +layout.svelte        # Program-scoped layout
│   │   │       ├── +layout.ts
│   │   │       ├── settings/
│   │   │       │   └── +page.svelte      # Program settings
│   │   │       ├── modules/
│   │   │       │   ├── new/
│   │   │       │   │   └── +page.svelte  # Create module
│   │   │       │   └── [moduleId]/
│   │   │       │       ├── +page.svelte  # Module overview
│   │   │       │       └── +page.ts
│   │   │       └── days/
│   │   │           ├── new/
│   │   │           │   └── +page.svelte  # Create day plan
│   │   │           └── [dayId]/
│   │   │               ├── +page.svelte  # Day Mission screen
│   │   │               ├── +page.ts
│   │   │               ├── attempt/
│   │   │               │   ├── +page.svelte    # Active attempt — working screen
│   │   │               │   └── +page.ts
│   │   │               ├── quiz/
│   │   │               │   ├── +page.svelte    # Quiz runner
│   │   │               │   └── +page.ts
│   │   │               ├── evidence/
│   │   │               │   ├── +page.svelte    # Evidence locker
│   │   │               │   └── +page.ts
│   │   │               └── history/
│   │   │                   ├── +page.svelte    # Attempt history + diff
│   │   │                   └── +page.ts
│   │   ├── analytics/
│   │   │   ├── +page.svelte              # Analytics dashboard
│   │   │   ├── +page.ts
│   │   │   ├── skills/
│   │   │   │   └── +page.svelte          # Skill radar deep dive
│   │   │   ├── weekly/
│   │   │   │   ├── +page.svelte          # Weekly reviews list
│   │   │   │   └── [weekId]/
│   │   │   │       └── +page.svelte      # Single weekly review
│   │   │   └── burndown/
│   │   │       └── +page.svelte          # Burndown chart
│   │   ├── search/
│   │   │   └── +page.svelte              # Full-text search results
│   │   ├── export/
│   │   │   └── +page.svelte              # Export center
│   │   └── settings/
│   │       └── +page.svelte              # App-level settings
├── src-tauri/                             # Rust backend
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json                  # Tauri v2 capability permissions
│   ├── icons/
│   │   ├── icon.icns
│   │   ├── icon.ico
│   │   ├── 32x32.png
│   │   ├── 128x128.png
│   │   └── 128x128@2x.png
│   └── src/
│       ├── main.rs                        # Tauri entry point
│       ├── lib.rs                         # Module declarations + app builder
│       ├── error.rs                       # Centralized error types (thiserror)
│       ├── db/
│       │   ├── mod.rs                     # DB module — pool init, migrations
│       │   ├── migrations/                # SQLx migrations directory
│       │   │   ├── 001_create_programs.sql
│       │   │   ├── 002_create_modules.sql
│       │   │   ├── 003_create_day_plans.sql
│       │   │   ├── 004_create_day_attempts.sql
│       │   │   ├── 005_create_checklists.sql
│       │   │   ├── 006_create_quizzes.sql
│       │   │   ├── 007_create_evidence.sql
│       │   │   ├── 008_create_bug_logs.sql
│       │   │   ├── 009_create_skill_scores.sql
│       │   │   ├── 010_create_weekly_reviews.sql
│       │   │   ├── 011_create_exercise_entries.sql
│       │   │   ├── 012_create_concept_tags.sql
│       │   │   ├── 013_create_spaced_repetition.sql
│       │   │   ├── 014_create_dependencies.sql
│       │   │   ├── 015_create_time_logs.sql
│       │   │   ├── 016_create_streaks.sql
│       │   │   ├── 017_create_badges.sql
│       │   │   ├── 018_create_fts_index.sql
│       │   │   └── 019_create_settings.sql
│       │   └── models/
│       │       ├── mod.rs
│       │       ├── program.rs
│       │       ├── module.rs
│       │       ├── day_plan.rs
│       │       ├── day_attempt.rs
│       │       ├── checklist.rs
│       │       ├── quiz.rs
│       │       ├── evidence.rs
│       │       ├── bug_log.rs
│       │       ├── skill_score.rs
│       │       ├── weekly_review.rs
│       │       ├── exercise_entry.rs
│       │       ├── concept_tag.rs
│       │       ├── spaced_repetition.rs
│       │       ├── dependency.rs
│       │       ├── time_log.rs
│       │       ├── streak.rs
│       │       ├── badge.rs
│       │       └── settings.rs
│       ├── commands/
│       │   ├── mod.rs                     # Command module — registers all commands
│       │   ├── programs.rs               # Program CRUD commands
│       │   ├── modules.rs                # Module CRUD commands
│       │   ├── days.rs                   # Day plan CRUD commands
│       │   ├── attempts.rs              # Day attempt CRUD commands
│       │   ├── quizzes.rs               # Quiz commands
│       │   ├── evidence.rs              # Evidence/artifact commands
│       │   ├── analytics.rs             # Analytics + scoring commands
│       │   ├── search.rs                # Full-text search commands
│       │   ├── export.rs                # Export commands (PDF, JSON, CSV)
│       │   ├── files.rs                 # File system operations
│       │   ├── spaced_repetition.rs     # Spaced repetition engine
│       │   └── settings.rs              # App settings commands
│       ├── services/
│       │   ├── mod.rs
│       │   ├── scoring.rs               # Score calculation service
│       │   ├── streak.rs                # Streak tracking service
│       │   ├── badge.rs                 # Badge evaluation service
│       │   ├── review.rs                # Weekly review generation service
│       │   ├── spaced_repetition.rs     # SM-2 algorithm service
│       │   ├── dependency.rs            # Dependency graph resolution
│       │   ├── search_index.rs          # Tantivy FTS index management
│       │   ├── export_pdf.rs            # PDF generation service
│       │   ├── export_data.rs           # JSON/CSV export service
│       │   ├── import.rs                # Program/data import service
│       │   └── autosave.rs              # Draft autosave service
│       └── utils/
│           ├── mod.rs
│           ├── fs.rs                     # File system helpers
│           ├── time.rs                   # Time/date utilities
│           └── validation.rs            # Input validation
├── static/
│   └── favicon.png
├── tailwind.config.ts
├── tsconfig.json
├── svelte.config.js
├── vite.config.ts
├── postcss.config.js
├── .prettierrc
├── .eslintrc.cjs
└── pnpm-lock.yaml
```

---

## 4. DATABASE SCHEMA — SQLITE via SQLx

All IDs are UUIDv4 stored as TEXT. All timestamps are ISO 8601 TEXT. SQLite FTS5 for full-text search.

### Migration 001 — Programs

```sql
CREATE TABLE IF NOT EXISTS programs (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    target_days INTEGER NOT NULL DEFAULT 40,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'paused', 'completed', 'archived')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_programs_status ON programs(status);
CREATE INDEX idx_programs_created_at ON programs(created_at);
```

### Migration 002 — Modules

```sql
CREATE TABLE IF NOT EXISTS modules (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    order_index INTEGER NOT NULL DEFAULT 0,
    color TEXT NOT NULL DEFAULT '#6366F1',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, order_index)
);

CREATE INDEX idx_modules_program ON modules(program_id);
CREATE INDEX idx_modules_order ON modules(program_id, order_index);
```

### Migration 003 — Day Plans

```sql
CREATE TABLE IF NOT EXISTS day_plans (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    module_id TEXT NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    day_number INTEGER NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),

    -- Content fields (Markdown)
    syntax_targets TEXT NOT NULL DEFAULT '',
    implementation_brief TEXT NOT NULL DEFAULT '',
    files_to_create TEXT NOT NULL DEFAULT '',
    success_criteria TEXT NOT NULL DEFAULT '',
    stretch_challenge TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',

    -- Timing
    estimated_minutes INTEGER NOT NULL DEFAULT 60,
    memory_rebuild_minutes INTEGER NOT NULL DEFAULT 15,

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, day_number, version)
);

CREATE INDEX idx_day_plans_program ON day_plans(program_id);
CREATE INDEX idx_day_plans_module ON day_plans(module_id);
CREATE INDEX idx_day_plans_day_number ON day_plans(program_id, day_number);
```

### Migration 004 — Day Attempts (DayRun)

```sql
CREATE TABLE IF NOT EXISTS day_attempts (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    day_plan_version INTEGER NOT NULL DEFAULT 1,
    attempt_number INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'in_progress' CHECK (status IN ('in_progress', 'submitted', 'blocked', 'passed', 'mastery')),

    -- Score card
    score_implementation INTEGER NOT NULL DEFAULT 0 CHECK (score_implementation >= 0 AND score_implementation <= 40),
    score_code_quality INTEGER NOT NULL DEFAULT 0 CHECK (score_code_quality >= 0 AND score_code_quality <= 20),
    score_accessibility INTEGER NOT NULL DEFAULT 0 CHECK (score_accessibility >= 0 AND score_accessibility <= 15),
    score_performance INTEGER NOT NULL DEFAULT 0 CHECK (score_performance >= 0 AND score_performance <= 15),
    score_quiz INTEGER NOT NULL DEFAULT 0 CHECK (score_quiz >= 0 AND score_quiz <= 10),
    total_score INTEGER GENERATED ALWAYS AS (
        score_implementation + score_code_quality + score_accessibility + score_performance + score_quiz
    ) STORED,

    -- Memory rebuild
    memory_rebuild_completed INTEGER NOT NULL DEFAULT 0,
    memory_rebuild_passed INTEGER NOT NULL DEFAULT 0,
    memory_rebuild_notes TEXT NOT NULL DEFAULT '',

    -- Reflection
    what_broke TEXT NOT NULL DEFAULT '',
    why_broke TEXT NOT NULL DEFAULT '',
    how_fixed TEXT NOT NULL DEFAULT '',
    refactor_tomorrow TEXT NOT NULL DEFAULT '',
    daily_summary TEXT NOT NULL DEFAULT '',

    -- Exercise work
    exercise_notes TEXT NOT NULL DEFAULT '',
    code_snapshot TEXT NOT NULL DEFAULT '',

    -- Timing
    started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    submitted_at TEXT,
    actual_minutes INTEGER NOT NULL DEFAULT 0,

    -- Draft/autosave
    is_draft INTEGER NOT NULL DEFAULT 1,
    last_autosave TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(day_plan_id, attempt_number)
);

CREATE INDEX idx_day_attempts_plan ON day_attempts(day_plan_id);
CREATE INDEX idx_day_attempts_status ON day_attempts(status);
CREATE INDEX idx_day_attempts_score ON day_attempts(total_score);
CREATE INDEX idx_day_attempts_submitted ON day_attempts(submitted_at);
```

### Migration 005 — Checklists

```sql
CREATE TABLE IF NOT EXISTS checklist_items (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    label TEXT NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS attempt_checklist (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    checklist_item_id TEXT NOT NULL REFERENCES checklist_items(id) ON DELETE CASCADE,
    is_completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,
    UNIQUE(attempt_id, checklist_item_id)
);

CREATE INDEX idx_checklist_items_plan ON checklist_items(day_plan_id);
CREATE INDEX idx_attempt_checklist_attempt ON attempt_checklist(attempt_id);
```

### Migration 006 — Quizzes

```sql
CREATE TABLE IF NOT EXISTS quiz_questions (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    question_type TEXT NOT NULL DEFAULT 'short_answer' CHECK (question_type IN ('short_answer', 'multiple_choice', 'code_prompt', 'reflection')),
    correct_answer TEXT NOT NULL DEFAULT '',
    options_json TEXT NOT NULL DEFAULT '[]',
    order_index INTEGER NOT NULL DEFAULT 0,
    points INTEGER NOT NULL DEFAULT 1,
    time_limit_seconds INTEGER NOT NULL DEFAULT 120,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES quiz_questions(id) ON DELETE CASCADE,
    user_answer TEXT NOT NULL DEFAULT '',
    is_correct INTEGER,
    score INTEGER NOT NULL DEFAULT 0,
    time_taken_seconds INTEGER NOT NULL DEFAULT 0,
    answered_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(attempt_id, question_id)
);

CREATE INDEX idx_quiz_questions_plan ON quiz_questions(day_plan_id);
CREATE INDEX idx_quiz_attempts_attempt ON quiz_attempts(attempt_id);
```

### Migration 007 — Evidence / Artifacts

```sql
CREATE TABLE IF NOT EXISTS artifacts (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    artifact_type TEXT NOT NULL CHECK (artifact_type IN ('file', 'screenshot', 'link', 'code_snippet', 'markdown_note')),
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',

    -- For files/screenshots
    file_path TEXT,
    file_name TEXT,
    file_size_bytes INTEGER,
    mime_type TEXT,

    -- For links
    url TEXT,

    -- For code snippets
    code_content TEXT,
    code_language TEXT,

    -- For markdown notes
    markdown_content TEXT,

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_artifacts_attempt ON artifacts(attempt_id);
CREATE INDEX idx_artifacts_type ON artifacts(artifact_type);
```

### Migration 008 — Bug Logs

```sql
CREATE TABLE IF NOT EXISTS bug_logs (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    symptom TEXT NOT NULL,
    root_cause TEXT NOT NULL DEFAULT '',
    fix TEXT NOT NULL DEFAULT '',
    prevention_rule TEXT NOT NULL DEFAULT '',
    severity TEXT NOT NULL DEFAULT 'medium' CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    category TEXT NOT NULL DEFAULT 'general',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_bug_logs_attempt ON bug_logs(attempt_id);
CREATE INDEX idx_bug_logs_category ON bug_logs(category);
CREATE INDEX idx_bug_logs_severity ON bug_logs(severity);
```

### Migration 009 — Skill Scores

```sql
CREATE TABLE IF NOT EXISTS skill_scores (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    domain TEXT NOT NULL,
    score REAL NOT NULL DEFAULT 0.0 CHECK (score >= 0.0 AND score <= 100.0),
    data_points INTEGER NOT NULL DEFAULT 0,
    last_assessed TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, domain)
);

CREATE INDEX idx_skill_scores_program ON skill_scores(program_id);
```

### Migration 010 — Weekly Reviews

```sql
CREATE TABLE IF NOT EXISTS weekly_reviews (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    week_number INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,

    -- Auto-generated metrics
    days_completed INTEGER NOT NULL DEFAULT 0,
    days_blocked INTEGER NOT NULL DEFAULT 0,
    average_score REAL NOT NULL DEFAULT 0.0,
    best_score INTEGER NOT NULL DEFAULT 0,
    worst_score INTEGER NOT NULL DEFAULT 0,
    total_time_minutes INTEGER NOT NULL DEFAULT 0,

    -- Concept analysis (JSON)
    strong_concepts_json TEXT NOT NULL DEFAULT '[]',
    weak_concepts_json TEXT NOT NULL DEFAULT '[]',
    replay_recommendations_json TEXT NOT NULL DEFAULT '[]',

    -- Manual notes
    summary TEXT NOT NULL DEFAULT '',
    goals_next_week TEXT NOT NULL DEFAULT '',

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, week_number)
);

CREATE INDEX idx_weekly_reviews_program ON weekly_reviews(program_id);
```

### Migration 011 — Exercise Entries

```sql
CREATE TABLE IF NOT EXISTS exercise_entries (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    entry_type TEXT NOT NULL CHECK (entry_type IN ('code', 'markdown', 'mixed')),
    title TEXT NOT NULL DEFAULT '',
    content TEXT NOT NULL DEFAULT '',
    language TEXT,
    order_index INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_exercise_entries_attempt ON exercise_entries(attempt_id);
```

### Migration 012 — Concept Tags

```sql
CREATE TABLE IF NOT EXISTS concept_tags (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    domain TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#6366F1',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE TABLE IF NOT EXISTS day_plan_tags (
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES concept_tags(id) ON DELETE CASCADE,
    PRIMARY KEY(day_plan_id, tag_id)
);

CREATE INDEX idx_concept_tags_domain ON concept_tags(domain);
CREATE INDEX idx_day_plan_tags_plan ON day_plan_tags(day_plan_id);
CREATE INDEX idx_day_plan_tags_tag ON day_plan_tags(tag_id);
```

### Migration 013 — Spaced Repetition

```sql
CREATE TABLE IF NOT EXISTS spaced_repetition (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    tag_id TEXT REFERENCES concept_tags(id) ON DELETE SET NULL,

    -- SM-2 algorithm fields
    easiness_factor REAL NOT NULL DEFAULT 2.5,
    interval_days INTEGER NOT NULL DEFAULT 1,
    repetitions INTEGER NOT NULL DEFAULT 0,
    next_review_date TEXT NOT NULL,
    last_review_date TEXT,
    last_quality INTEGER,

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(day_plan_id)
);

CREATE INDEX idx_spaced_repetition_next_review ON spaced_repetition(next_review_date);
CREATE INDEX idx_spaced_repetition_plan ON spaced_repetition(day_plan_id);
```

### Migration 014 — Dependencies

```sql
CREATE TABLE IF NOT EXISTS day_dependencies (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    depends_on_day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    dependency_type TEXT NOT NULL DEFAULT 'prerequisite' CHECK (dependency_type IN ('prerequisite', 'recommended', 'related')),
    minimum_score INTEGER NOT NULL DEFAULT 70,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(day_plan_id, depends_on_day_plan_id),
    CHECK(day_plan_id != depends_on_day_plan_id)
);

CREATE INDEX idx_day_dependencies_plan ON day_dependencies(day_plan_id);
CREATE INDEX idx_day_dependencies_depends ON day_dependencies(depends_on_day_plan_id);
```

### Migration 015 — Time Logs

```sql
CREATE TABLE IF NOT EXISTS time_logs (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    session_type TEXT NOT NULL DEFAULT 'implementation' CHECK (session_type IN ('implementation', 'memory_rebuild', 'quiz', 'review', 'exploration')),
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    notes TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_time_logs_attempt ON time_logs(attempt_id);
CREATE INDEX idx_time_logs_session_type ON time_logs(session_type);
```

### Migration 016 — Streaks

```sql
CREATE TABLE IF NOT EXISTS streaks (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    current_streak INTEGER NOT NULL DEFAULT 0,
    longest_streak INTEGER NOT NULL DEFAULT 0,
    last_active_date TEXT,
    streak_freezes_used INTEGER NOT NULL DEFAULT 0,
    streak_freezes_available INTEGER NOT NULL DEFAULT 2,
    freeze_reset_month TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id)
);
```

### Migration 017 — Badges

```sql
CREATE TABLE IF NOT EXISTS badges (
    id TEXT PRIMARY KEY NOT NULL,
    badge_type TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    icon TEXT NOT NULL DEFAULT 'ph:trophy-bold',
    earned_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    program_id TEXT REFERENCES programs(id) ON DELETE CASCADE,
    day_plan_id TEXT REFERENCES day_plans(id) ON DELETE SET NULL,
    metadata_json TEXT NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_badges_type ON badges(badge_type);
CREATE INDEX idx_badges_program ON badges(program_id);
```

### Migration 018 — Full-Text Search (FTS5)

```sql
CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
    entity_id,
    entity_type,
    title,
    content,
    tags,
    tokenize='porter unicode61'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS trg_day_plans_fts_insert AFTER INSERT ON day_plans BEGIN
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'day_plan', NEW.title, NEW.syntax_targets || ' ' || NEW.implementation_brief || ' ' || NEW.notes, '');
END;

CREATE TRIGGER IF NOT EXISTS trg_day_plans_fts_update AFTER UPDATE ON day_plans BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'day_plan';
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'day_plan', NEW.title, NEW.syntax_targets || ' ' || NEW.implementation_brief || ' ' || NEW.notes, '');
END;

CREATE TRIGGER IF NOT EXISTS trg_day_plans_fts_delete AFTER DELETE ON day_plans BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'day_plan';
END;

CREATE TRIGGER IF NOT EXISTS trg_exercise_entries_fts_insert AFTER INSERT ON exercise_entries BEGIN
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'exercise', NEW.title, NEW.content, COALESCE(NEW.language, ''));
END;

CREATE TRIGGER IF NOT EXISTS trg_exercise_entries_fts_update AFTER UPDATE ON exercise_entries BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'exercise';
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'exercise', NEW.title, NEW.content, COALESCE(NEW.language, ''));
END;

CREATE TRIGGER IF NOT EXISTS trg_exercise_entries_fts_delete AFTER DELETE ON exercise_entries BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'exercise';
END;

CREATE TRIGGER IF NOT EXISTS trg_bug_logs_fts_insert AFTER INSERT ON bug_logs BEGIN
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'bug_log', NEW.symptom, NEW.root_cause || ' ' || NEW.fix || ' ' || NEW.prevention_rule, NEW.category);
END;

CREATE TRIGGER IF NOT EXISTS trg_bug_logs_fts_update AFTER UPDATE ON bug_logs BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'bug_log';
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'bug_log', NEW.symptom, NEW.root_cause || ' ' || NEW.fix || ' ' || NEW.prevention_rule, NEW.category);
END;

CREATE TRIGGER IF NOT EXISTS trg_bug_logs_fts_delete AFTER DELETE ON bug_logs BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'bug_log';
END;

CREATE TRIGGER IF NOT EXISTS trg_day_attempts_fts_insert AFTER INSERT ON day_attempts BEGIN
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'attempt', 'Attempt', NEW.exercise_notes || ' ' || NEW.daily_summary || ' ' || NEW.what_broke || ' ' || NEW.how_fixed, '');
END;

CREATE TRIGGER IF NOT EXISTS trg_day_attempts_fts_update AFTER UPDATE ON day_attempts BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'attempt';
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'attempt', 'Attempt', NEW.exercise_notes || ' ' || NEW.daily_summary || ' ' || NEW.what_broke || ' ' || NEW.how_fixed, '');
END;

CREATE TRIGGER IF NOT EXISTS trg_day_attempts_fts_delete AFTER DELETE ON day_attempts BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'attempt';
END;

CREATE TRIGGER IF NOT EXISTS trg_artifacts_fts_insert AFTER INSERT ON artifacts BEGIN
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'artifact', NEW.title, COALESCE(NEW.description, '') || ' ' || COALESCE(NEW.code_content, '') || ' ' || COALESCE(NEW.markdown_content, ''), COALESCE(NEW.code_language, ''));
END;

CREATE TRIGGER IF NOT EXISTS trg_artifacts_fts_update AFTER UPDATE ON artifacts BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'artifact';
    INSERT INTO search_index(entity_id, entity_type, title, content, tags)
    VALUES (NEW.id, 'artifact', NEW.title, COALESCE(NEW.description, '') || ' ' || COALESCE(NEW.code_content, '') || ' ' || COALESCE(NEW.markdown_content, ''), COALESCE(NEW.code_language, ''));
END;

CREATE TRIGGER IF NOT EXISTS trg_artifacts_fts_delete AFTER DELETE ON artifacts BEGIN
    DELETE FROM search_index WHERE entity_id = OLD.id AND entity_type = 'artifact';
END;
```

### Migration 019 — Settings

```sql
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Default settings
INSERT OR IGNORE INTO settings (key, value) VALUES
    ('theme', '"dark"'),
    ('autosave_interval_ms', '5000'),
    ('default_session_minutes', '60'),
    ('memory_rebuild_minutes', '15'),
    ('blocked_threshold', '70'),
    ('mastery_threshold', '95'),
    ('streak_freezes_per_month', '2'),
    ('daily_reminder_enabled', 'true'),
    ('daily_reminder_time', '"09:00"'),
    ('spaced_repetition_enabled', 'true'),
    ('font_size', '14'),
    ('editor_theme', '"one-dark"'),
    ('sidebar_collapsed', 'false');
```

---

## 5. RUST BACKEND — COMPLETE IPC COMMAND SPECIFICATION

Every Tauri command is invoked from the frontend via `invoke()`. Every command returns `Result<T, String>` where errors are serialized for the frontend. All commands use the managed SQLx pool.

### 5.1 Program Commands

```rust
// commands/programs.rs

#[tauri::command]
async fn create_program(state: State<'_, AppState>, title: String, description: String, target_days: i32) -> Result<Program, String>;

#[tauri::command]
async fn get_program(state: State<'_, AppState>, id: String) -> Result<Program, String>;

#[tauri::command]
async fn list_programs(state: State<'_, AppState>) -> Result<Vec<ProgramSummary>, String>;

#[tauri::command]
async fn update_program(state: State<'_, AppState>, id: String, title: Option<String>, description: Option<String>, status: Option<String>) -> Result<Program, String>;

#[tauri::command]
async fn delete_program(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn duplicate_program(state: State<'_, AppState>, id: String, new_title: String) -> Result<Program, String>;

#[tauri::command]
async fn get_program_stats(state: State<'_, AppState>, id: String) -> Result<ProgramStats, String>;
```

### 5.2 Module Commands

```rust
// commands/modules.rs

#[tauri::command]
async fn create_module(state: State<'_, AppState>, program_id: String, title: String, description: String, color: String) -> Result<Module, String>;

#[tauri::command]
async fn get_module(state: State<'_, AppState>, id: String) -> Result<Module, String>;

#[tauri::command]
async fn list_modules(state: State<'_, AppState>, program_id: String) -> Result<Vec<Module>, String>;

#[tauri::command]
async fn update_module(state: State<'_, AppState>, id: String, title: Option<String>, description: Option<String>, color: Option<String>) -> Result<Module, String>;

#[tauri::command]
async fn delete_module(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn reorder_modules(state: State<'_, AppState>, program_id: String, module_ids: Vec<String>) -> Result<(), String>;
```

### 5.3 Day Plan Commands

```rust
// commands/days.rs

#[tauri::command]
async fn create_day_plan(state: State<'_, AppState>, input: CreateDayPlanInput) -> Result<DayPlan, String>;

#[tauri::command]
async fn get_day_plan(state: State<'_, AppState>, id: String) -> Result<DayPlanFull, String>;
// DayPlanFull includes checklist_items, quiz_questions, concept_tags, dependencies

#[tauri::command]
async fn list_day_plans(state: State<'_, AppState>, program_id: String) -> Result<Vec<DayPlanSummary>, String>;

#[tauri::command]
async fn list_day_plans_by_module(state: State<'_, AppState>, module_id: String) -> Result<Vec<DayPlanSummary>, String>;

#[tauri::command]
async fn update_day_plan(state: State<'_, AppState>, id: String, input: UpdateDayPlanInput) -> Result<DayPlan, String>;
// Updating a published day plan increments version

#[tauri::command]
async fn delete_day_plan(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn reorder_day_plans(state: State<'_, AppState>, program_id: String, day_plan_ids: Vec<String>) -> Result<(), String>;

#[tauri::command]
async fn duplicate_day_plan(state: State<'_, AppState>, id: String) -> Result<DayPlan, String>;

// Checklist management
#[tauri::command]
async fn add_checklist_item(state: State<'_, AppState>, day_plan_id: String, label: String, is_required: bool) -> Result<ChecklistItem, String>;

#[tauri::command]
async fn update_checklist_item(state: State<'_, AppState>, id: String, label: Option<String>, is_required: Option<bool>) -> Result<ChecklistItem, String>;

#[tauri::command]
async fn delete_checklist_item(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn reorder_checklist_items(state: State<'_, AppState>, day_plan_id: String, item_ids: Vec<String>) -> Result<(), String>;

// Quiz management
#[tauri::command]
async fn add_quiz_question(state: State<'_, AppState>, input: CreateQuizQuestionInput) -> Result<QuizQuestion, String>;

#[tauri::command]
async fn update_quiz_question(state: State<'_, AppState>, id: String, input: UpdateQuizQuestionInput) -> Result<QuizQuestion, String>;

#[tauri::command]
async fn delete_quiz_question(state: State<'_, AppState>, id: String) -> Result<(), String>;

// Concept tags
#[tauri::command]
async fn add_tag_to_day(state: State<'_, AppState>, day_plan_id: String, tag_id: String) -> Result<(), String>;

#[tauri::command]
async fn remove_tag_from_day(state: State<'_, AppState>, day_plan_id: String, tag_id: String) -> Result<(), String>;

#[tauri::command]
async fn create_concept_tag(state: State<'_, AppState>, name: String, domain: String, color: String) -> Result<ConceptTag, String>;

#[tauri::command]
async fn list_concept_tags(state: State<'_, AppState>) -> Result<Vec<ConceptTag>, String>;

// Dependencies
#[tauri::command]
async fn add_dependency(state: State<'_, AppState>, day_plan_id: String, depends_on_id: String, dep_type: String, min_score: i32) -> Result<(), String>;

#[tauri::command]
async fn remove_dependency(state: State<'_, AppState>, day_plan_id: String, depends_on_id: String) -> Result<(), String>;

#[tauri::command]
async fn check_dependencies(state: State<'_, AppState>, day_plan_id: String) -> Result<Vec<DependencyStatus>, String>;
// Returns list of dependencies with current met/unmet status
```

### 5.4 Day Attempt Commands

```rust
// commands/attempts.rs

#[tauri::command]
async fn start_attempt(state: State<'_, AppState>, day_plan_id: String) -> Result<DayAttempt, String>;
// Creates new attempt, increments attempt_number, starts time log

#[tauri::command]
async fn get_attempt(state: State<'_, AppState>, id: String) -> Result<DayAttemptFull, String>;
// Full attempt with checklist state, quiz answers, artifacts, bug logs, exercise entries

#[tauri::command]
async fn get_current_attempt(state: State<'_, AppState>, day_plan_id: String) -> Result<Option<DayAttemptFull>, String>;
// Returns latest in_progress attempt if exists

#[tauri::command]
async fn list_attempts(state: State<'_, AppState>, day_plan_id: String) -> Result<Vec<DayAttemptSummary>, String>;

#[tauri::command]
async fn autosave_attempt(state: State<'_, AppState>, id: String, input: AutosaveInput) -> Result<(), String>;
// Updates exercise_notes, code_snapshot, reflection fields, etc. — called on interval

#[tauri::command]
async fn submit_attempt(state: State<'_, AppState>, id: String, scores: ScoreCardInput) -> Result<DayAttempt, String>;
// Validates scores, calculates total, determines status (blocked/passed/mastery)
// Updates streak, triggers badge evaluation, updates skill scores
// Creates spaced repetition entry

#[tauri::command]
async fn toggle_checklist_item(state: State<'_, AppState>, attempt_id: String, checklist_item_id: String) -> Result<bool, String>;

// Memory rebuild
#[tauri::command]
async fn start_memory_rebuild(state: State<'_, AppState>, attempt_id: String) -> Result<TimeLog, String>;

#[tauri::command]
async fn complete_memory_rebuild(state: State<'_, AppState>, attempt_id: String, passed: bool, notes: String) -> Result<DayAttempt, String>;

// Bug logs
#[tauri::command]
async fn add_bug_log(state: State<'_, AppState>, attempt_id: String, input: CreateBugLogInput) -> Result<BugLog, String>;

#[tauri::command]
async fn update_bug_log(state: State<'_, AppState>, id: String, input: UpdateBugLogInput) -> Result<BugLog, String>;

#[tauri::command]
async fn delete_bug_log(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn list_bug_logs(state: State<'_, AppState>, attempt_id: String) -> Result<Vec<BugLog>, String>;

// Exercise entries
#[tauri::command]
async fn add_exercise_entry(state: State<'_, AppState>, attempt_id: String, input: CreateExerciseEntryInput) -> Result<ExerciseEntry, String>;

#[tauri::command]
async fn update_exercise_entry(state: State<'_, AppState>, id: String, input: UpdateExerciseEntryInput) -> Result<ExerciseEntry, String>;

#[tauri::command]
async fn delete_exercise_entry(state: State<'_, AppState>, id: String) -> Result<(), String>;

#[tauri::command]
async fn reorder_exercise_entries(state: State<'_, AppState>, attempt_id: String, entry_ids: Vec<String>) -> Result<(), String>;

// Quiz execution
#[tauri::command]
async fn submit_quiz_answer(state: State<'_, AppState>, attempt_id: String, question_id: String, answer: String, time_taken: i32) -> Result<QuizAttempt, String>;

#[tauri::command]
async fn get_quiz_results(state: State<'_, AppState>, attempt_id: String) -> Result<QuizResults, String>;
```

### 5.5 Evidence / Artifact Commands

```rust
// commands/evidence.rs

#[tauri::command]
async fn upload_artifact(state: State<'_, AppState>, attempt_id: String, file_path: String, title: String, description: String) -> Result<Artifact, String>;
// Copies file to app_data_dir/artifacts/{attempt_id}/{uuid}_{filename}
// Stores metadata in DB

#[tauri::command]
async fn add_link_artifact(state: State<'_, AppState>, attempt_id: String, url: String, title: String, description: String) -> Result<Artifact, String>;

#[tauri::command]
async fn add_code_artifact(state: State<'_, AppState>, attempt_id: String, code: String, language: String, title: String) -> Result<Artifact, String>;

#[tauri::command]
async fn add_note_artifact(state: State<'_, AppState>, attempt_id: String, markdown: String, title: String) -> Result<Artifact, String>;

#[tauri::command]
async fn delete_artifact(state: State<'_, AppState>, id: String) -> Result<(), String>;
// Also deletes file from disk if file-based

#[tauri::command]
async fn list_artifacts(state: State<'_, AppState>, attempt_id: String) -> Result<Vec<Artifact>, String>;

#[tauri::command]
async fn open_artifact_file(state: State<'_, AppState>, id: String) -> Result<(), String>;
// Opens file with system default application via tauri-plugin-shell
```

### 5.6 Analytics Commands

```rust
// commands/analytics.rs

#[tauri::command]
async fn get_dashboard_data(state: State<'_, AppState>, program_id: String) -> Result<DashboardData, String>;
// Returns: today's day, progress %, streak, quality trend, blocked alerts, upcoming days

#[tauri::command]
async fn get_skill_radar(state: State<'_, AppState>, program_id: String) -> Result<Vec<SkillScore>, String>;

#[tauri::command]
async fn get_weekly_review(state: State<'_, AppState>, program_id: String, week_number: i32) -> Result<WeeklyReview, String>;

#[tauri::command]
async fn generate_weekly_review(state: State<'_, AppState>, program_id: String) -> Result<WeeklyReview, String>;
// Auto-generates review for current week from attempt data

#[tauri::command]
async fn get_burndown_data(state: State<'_, AppState>, program_id: String) -> Result<BurndownData, String>;

#[tauri::command]
async fn get_concept_heatmap(state: State<'_, AppState>, program_id: String) -> Result<Vec<ConceptHeatmapEntry>, String>;

#[tauri::command]
async fn get_score_history(state: State<'_, AppState>, program_id: String, days: Option<i32>) -> Result<Vec<ScoreHistoryPoint>, String>;

#[tauri::command]
async fn get_time_analytics(state: State<'_, AppState>, program_id: String) -> Result<TimeAnalytics, String>;

#[tauri::command]
async fn get_bug_analytics(state: State<'_, AppState>, program_id: String) -> Result<BugAnalytics, String>;
// Category frequency, severity distribution, recurring patterns

#[tauri::command]
async fn get_streak_data(state: State<'_, AppState>, program_id: String) -> Result<StreakData, String>;

#[tauri::command]
async fn use_streak_freeze(state: State<'_, AppState>, program_id: String) -> Result<StreakData, String>;

#[tauri::command]
async fn get_badges(state: State<'_, AppState>, program_id: Option<String>) -> Result<Vec<Badge>, String>;

#[tauri::command]
async fn get_forgetting_curve_alerts(state: State<'_, AppState>, program_id: String) -> Result<Vec<ForgettingCurveAlert>, String>;
```

### 5.7 Spaced Repetition Commands

```rust
// commands/spaced_repetition.rs

#[tauri::command]
async fn get_due_reviews(state: State<'_, AppState>, program_id: String) -> Result<Vec<SpacedRepetitionItem>, String>;
// Returns day plans due for review today

#[tauri::command]
async fn record_review(state: State<'_, AppState>, day_plan_id: String, quality: i32) -> Result<SpacedRepetitionItem, String>;
// quality: 0-5 (SM-2 scale), recalculates interval and next review date

#[tauri::command]
async fn get_review_schedule(state: State<'_, AppState>, program_id: String, days_ahead: i32) -> Result<Vec<ReviewScheduleEntry>, String>;
// Shows upcoming reviews for the next N days
```

### 5.8 Search Commands

```rust
// commands/search.rs

#[tauri::command]
async fn search(state: State<'_, AppState>, query: String, entity_types: Option<Vec<String>>, program_id: Option<String>, limit: Option<i32>) -> Result<Vec<SearchResult>, String>;
// Searches FTS5 index, returns ranked results with snippets
// entity_types filter: day_plan, exercise, bug_log, attempt, artifact

#[tauri::command]
async fn rebuild_search_index(state: State<'_, AppState>) -> Result<i32, String>;
// Rebuilds FTS5 index from scratch, returns number of entries indexed
```

### 5.9 Export Commands

```rust
// commands/export.rs

#[tauri::command]
async fn export_program_pdf(state: State<'_, AppState>, program_id: String, output_path: String) -> Result<String, String>;
// Generates comprehensive PDF report with all analytics, scores, reflections

#[tauri::command]
async fn export_weekly_report_pdf(state: State<'_, AppState>, program_id: String, week_number: i32, output_path: String) -> Result<String, String>;

#[tauri::command]
async fn export_program_json(state: State<'_, AppState>, program_id: String, output_path: String) -> Result<String, String>;
// Full program data export — can be re-imported

#[tauri::command]
async fn export_program_csv(state: State<'_, AppState>, program_id: String, output_path: String) -> Result<String, String>;
// Flat CSV of all day attempts with scores

#[tauri::command]
async fn import_program_json(state: State<'_, AppState>, file_path: String) -> Result<Program, String>;
// Imports program from JSON export

#[tauri::command]
async fn export_day_plan_template(state: State<'_, AppState>, day_plan_id: String) -> Result<String, String>;
// Returns JSON template that can be used to create new day plans
```

### 5.10 File & Settings Commands

```rust
// commands/files.rs

#[tauri::command]
async fn pick_file(app: AppHandle) -> Result<Option<String>, String>;
// Opens native file picker dialog, returns selected path

#[tauri::command]
async fn pick_save_location(app: AppHandle, default_name: String) -> Result<Option<String>, String>;
// Opens native save dialog

#[tauri::command]
async fn get_app_data_dir(app: AppHandle) -> Result<String, String>;

// commands/settings.rs

#[tauri::command]
async fn get_setting(state: State<'_, AppState>, key: String) -> Result<String, String>;

#[tauri::command]
async fn set_setting(state: State<'_, AppState>, key: String, value: String) -> Result<(), String>;

#[tauri::command]
async fn get_all_settings(state: State<'_, AppState>) -> Result<HashMap<String, String>, String>;

// Time log commands
#[tauri::command]
async fn start_time_log(state: State<'_, AppState>, attempt_id: String, session_type: String) -> Result<TimeLog, String>;

#[tauri::command]
async fn stop_time_log(state: State<'_, AppState>, id: String) -> Result<TimeLog, String>;

#[tauri::command]
async fn get_active_time_log(state: State<'_, AppState>, attempt_id: String) -> Result<Option<TimeLog>, String>;
```

---

## 6. FRONTEND IPC LAYER — TypeScript Command Wrappers

Every Tauri command is wrapped in a type-safe TypeScript function.

```typescript
// src/lib/commands/programs.ts
import { invoke } from '@tauri-apps/api/core';
import type { Program, ProgramSummary, ProgramStats } from '$lib/types';

export async function createProgram(title: string, description: string, targetDays: number): Promise<Program> {
  return invoke<Program>('create_program', { title, description, targetDays });
}

export async function getProgram(id: string): Promise<Program> {
  return invoke<Program>('get_program', { id });
}

export async function listPrograms(): Promise<ProgramSummary[]> {
  return invoke<ProgramSummary[]>('list_programs');
}

// ... and so on for every command
```

**Every command module follows this exact pattern. No exceptions.**

---

## 7. SCORING ENGINE — COMPLETE SPECIFICATION

### Score Weights

| Category | Max Points | Weight |
|----------|-----------|--------|
| Implementation Completeness | 40 | 40% |
| Code Quality | 20 | 20% |
| Accessibility / UX | 15 | 15% |
| Performance / Resilience | 15 | 15% |
| Quiz | 10 | 10% |
| **Total** | **100** | **100%** |

### Quality Gates

| Score Range | Status | Behavior |
|-------------|--------|----------|
| 0–69 | `blocked` | Must replay day before advancing to dependent days |
| 70–84 | `passed` | Pass with improvement recommendations |
| 85–94 | `passed` | Strong pass |
| 95–100 | `mastery` | Mastery badge candidate |

### Memory Rebuild Impact

If `memory_rebuild_completed = true` but `memory_rebuild_passed = false`, the day status is downgraded to `blocked` regardless of the numerical score. This enforces genuine learning.

### Spaced Repetition — SM-2 Algorithm

```
After grading quality q (0-5):
  if q < 3: reset repetitions to 0, interval to 1
  else:
    if repetitions == 0: interval = 1
    if repetitions == 1: interval = 6
    else: interval = round(interval * easiness_factor)
    repetitions += 1

  easiness_factor = max(1.3, EF + (0.1 - (5 - q) * (0.08 + (5 - q) * 0.02)))
  next_review_date = today + interval days
```

Quality mapping from app scores:
- Total 0–49 → q = 0
- Total 50–59 → q = 1
- Total 60–69 → q = 2
- Total 70–79 → q = 3
- Total 80–89 → q = 4
- Total 90–100 → q = 5

---

## 8. BADGE DEFINITIONS

| Badge ID | Title | Condition |
|----------|-------|-----------|
| `first_day` | First Step | Complete first day attempt |
| `streak_7` | Week Warrior | 7-day streak |
| `streak_14` | Two-Week Titan | 14-day streak |
| `streak_30` | Monthly Machine | 30-day streak |
| `perfect_week` | Perfect Week | 7 consecutive days with score ≥ 85 |
| `zero_blockers_14` | Clean Sheet | 14 days with zero blocked status |
| `mastery_day` | Mastery Achieved | Score 95+ on a single day |
| `mastery_5` | Mastery Streak | 5 mastery-level days |
| `bug_hunter_100` | Bug Hunter | 100 bug log entries |
| `memory_master` | Memory Master | 10 consecutive passed memory rebuilds |
| `full_evidence` | Evidence King | 20 days with 3+ artifacts each |
| `program_complete` | Program Complete | All days in a program submitted with score ≥ 70 |
| `speed_demon` | Speed Demon | Complete a day under estimated time with score ≥ 85 |
| `comeback` | Comeback | Replay a blocked day and score ≥ 85 |

---

## 9. UI / UX DESIGN SPECIFICATION

### Design System

**Color Palette — Dark Mode Primary (default):**

```css
--bg-primary: #0A0A0B;         /* App background */
--bg-secondary: #111113;       /* Card/panel background */
--bg-tertiary: #1A1A1E;        /* Elevated surfaces */
--bg-hover: #222228;           /* Hover states */
--border-primary: #2A2A30;     /* Primary borders */
--border-secondary: #3A3A42;   /* Emphasized borders */
--text-primary: #FAFAFA;       /* Primary text */
--text-secondary: #A1A1AA;     /* Secondary text */
--text-tertiary: #71717A;      /* Muted text */
--accent-primary: #6366F1;     /* Indigo — primary actions */
--accent-success: #22C55E;     /* Green — pass/success */
--accent-warning: #F59E0B;     /* Amber — warnings/recommendations */
--accent-danger: #EF4444;      /* Red — blocked/errors */
--accent-info: #3B82F6;        /* Blue — informational */
--accent-mastery: #A855F7;     /* Purple — mastery level */
```

**Color Palette — Light Mode:**

```css
--bg-primary: #FFFFFF;
--bg-secondary: #F9FAFB;
--bg-tertiary: #F3F4F6;
--bg-hover: #E5E7EB;
--border-primary: #E5E7EB;
--border-secondary: #D1D5DB;
--text-primary: #111827;
--text-secondary: #6B7280;
--text-tertiary: #9CA3AF;
/* Accent colors stay the same */
```

**Typography:**

```css
--font-sans: 'Inter', system-ui, -apple-system, sans-serif;
--font-mono: 'JetBrains Mono', 'SF Mono', 'Fira Code', monospace;

--text-xs: 0.75rem;     /* 12px */
--text-sm: 0.875rem;    /* 14px */
--text-base: 1rem;      /* 16px */
--text-lg: 1.125rem;    /* 18px */
--text-xl: 1.25rem;     /* 20px */
--text-2xl: 1.5rem;     /* 24px */
--text-3xl: 1.875rem;   /* 30px */
```

**Spacing scale:** 4px base — 4, 8, 12, 16, 20, 24, 32, 40, 48, 64, 80, 96

**Border radius:** `rounded-md` (6px) for inputs, `rounded-lg` (8px) for cards, `rounded-xl` (12px) for modals

**Shadows (dark mode):** Use subtle borders instead of drop shadows. Elevation is communicated via background color stepping.

### Layout Structure

```
┌─────────────────────────────────────────────────────────┐
│ TopBar (macOS titlebar overlay + breadcrumbs + search)  │
├──────────┬──────────────────────────────────────────────┤
│          │                                              │
│ Sidebar  │              Main Content Area               │
│ 240px    │              (scrollable)                    │
│ fixed    │                                              │
│          │                                              │
│ - Logo   │                                              │
│ - Nav    │                                              │
│ - Active │                                              │
│   Program│                                              │
│ - Quick  │                                              │
│   Stats  │                                              │
│          │                                              │
└──────────┴──────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `⌘ + K` | Command palette / global search |
| `⌘ + N` | New day plan |
| `⌘ + S` | Force save / submit |
| `⌘ + Enter` | Submit current form |
| `⌘ + B` | Toggle sidebar |
| `⌘ + 1–5` | Navigate to main sections |
| `⌘ + Shift + T` | Start/stop session timer |
| `Esc` | Close modal / command palette |
| `⌘ + /` | Toggle code editor comment |

### Responsive Behavior (within window)

- **< 1024px width:** Sidebar collapses to icon-only (48px)
- **< 768px width:** Sidebar fully hidden, hamburger menu in topbar
- All content grids adapt: 3-col → 2-col → 1-col

### Loading States

Every data-fetching view must show:
1. Skeleton placeholders during load (never blank screens)
2. Error state with retry button on failure
3. Empty state with contextual CTA when no data exists

### Toast Notifications

- Auto-dismiss after 5 seconds for success/info
- Persist until dismissed for errors
- Stack from bottom-right, max 3 visible
- Include undo action where applicable (delete operations)

---

## 10. COMPLETE FEATURE SPECIFICATIONS

### 10.1 Dashboard (`/`)

**Data displayed:**
- Today's mission card: day number, title, module, status badge, "Start" or "Continue" CTA
- Progress ring: `days_completed / total_days` with percentage
- Current streak count + longest streak
- Quality score trend: sparkline of last 7 day scores
- Blocked alerts: list of days with score < 70 that block dependent days
- Spaced repetition due today: count of concepts due for review
- Recent activity feed: last 5 actions (submissions, uploads, reviews)
- Upcoming 3 days preview

### 10.2 Day Mission (`/programs/[id]/days/[dayId]`)

**Sections:**
1. **Header** — Day number, title, module badge, status, estimated time, last attempt score
2. **Dependency check** — Shows prerequisite status, blocks "Start Attempt" if unmet
3. **Syntax Targets** — Rendered markdown of what to learn
4. **Implementation Brief** — Rendered markdown of what to build
5. **Files to Create** — List of expected deliverables
6. **Success Criteria** — Rendered markdown
7. **Checklist** — Interactive task list
8. **Stretch Challenge** — Optional extension task
9. **Quiz Preview** — Shows number of questions and time estimate
10. **Attempt History** — Table of past attempts with scores, dates, delta

**Actions:**
- Start Attempt → creates new DayRun, navigates to attempt screen
- Replay Day → creates new attempt with incremented number
- Edit Day Plan → opens editor (if you're in authoring mode)

### 10.3 Active Attempt (`/programs/[id]/days/[dayId]/attempt`)

This is the main working screen. **Must support autosave.**

**Layout: Two-pane (adjustable split)**

Left pane: Day plan content (read-only reference)
Right pane: Your work area with tabs:

**Tab 1 — Exercise Work**
- CodeMirror 6 editor with language selection
- Multiple exercise entry blocks (add/remove/reorder)
- Markdown editor for notes
- Autosaves every 5 seconds

**Tab 2 — Checklist**
- Interactive checklist from day plan
- Progress indicator (completed / total)

**Tab 3 — Bug Log**
- Add bug entries: symptom, root cause, fix, prevention rule, severity, category
- List of current session's bugs

**Tab 4 — Evidence**
- Upload files (drag & drop zone)
- Add links (repo URLs, Loom, etc.)
- Add code snippets
- Add markdown notes

**Tab 5 — Quiz**
- Quiz runner with timer per question
- Submit answers, see results

**Footer bar:**
- Session timer (running clock)
- Autosave indicator ("Last saved 3s ago")
- Score card quick-entry
- Submit Attempt button

### 10.4 Evidence Locker (`/programs/[id]/days/[dayId]/evidence`)

- Gallery view of all artifacts across all attempts for this day
- Filter by type, attempt number
- Preview files inline (images, markdown, code)
- Open files in system default app

### 10.5 Analytics Dashboard (`/analytics`)

**Charts and visualizations:**
1. **Skill Radar** — ECharts radar chart with axes for each domain (HTML/A11y, CSS, JS, Svelte 5, SvelteKit, GSAP, TypeScript, etc.)
2. **Score Trend** — Line chart of total scores over time
3. **Concept Heatmap** — Grid showing concept tag × score intensity
4. **Burndown Chart** — Days remaining vs projected completion, with actual pace line
5. **Time Distribution** — Stacked bar: planned vs actual minutes per day
6. **Bug Category Distribution** — Pie/donut chart of bug categories
7. **Weekly Score Comparison** — Bar chart by week
8. **Streak Calendar** — GitHub-style contribution calendar showing daily activity

### 10.6 Weekly Review (`/analytics/weekly/[weekId]`)

Auto-generated review containing:
- Days completed this week
- Average, best, worst scores
- Total time invested
- Strong concepts (scored ≥ 85)
- Weak concepts (scored < 70)
- Replay recommendations
- Bug pattern analysis
- Manual notes field for personal reflection
- Goals for next week field

### 10.7 Program Manager (`/programs`)

- List all programs with status, progress, last active date
- Create new program with title, description
- Duplicate existing program (creates copy of all modules + day plans)
- Archive/delete programs
- Import program from JSON

### 10.8 Day Plan Editor (`/programs/[id]/days/new` or edit mode)

Full authoring interface:
- Title, module assignment, day number
- Markdown editors for: syntax targets, implementation brief, files to create, success criteria, stretch challenge, notes
- Checklist builder (add/remove/reorder items)
- Quiz builder (add questions with types, answers, time limits)
- Concept tag selector (multi-select from existing + create new)
- Dependency selector (choose prerequisite days + minimum score)
- Estimated time input
- Memory rebuild time input
- Save as draft / Publish
- Day plan templates — save current as template / load from template

### 10.9 Search (`/search`)

- Global search bar in topbar (⌘+K also opens command palette)
- Searches across: day plans, exercise entries, bug logs, attempt reflections, artifacts
- Results grouped by entity type
- Snippet highlighting
- Click result to navigate to source
- Filter by: entity type, program, date range, score range

### 10.10 Export Center (`/export`)

- Export full program as PDF report
- Export weekly report as PDF
- Export program data as JSON (re-importable)
- Export scores/attempts as CSV
- Export individual day plan as template JSON
- Native save dialog for choosing output location

### 10.11 Settings (`/settings`)

- Theme: dark / light / system
- Editor font size
- Editor theme (one-dark, default, etc.)
- Autosave interval
- Default session time
- Memory rebuild time limit
- Blocked score threshold
- Mastery score threshold
- Streak freeze count per month
- Daily reminder enable/disable + time
- Spaced repetition enable/disable
- Data management: export all data, import data, clear all data (with confirmation)
- About: version, build info

### 10.12 Command Palette

Triggered by `⌘+K`:
- Fuzzy search across all navigation destinations
- Quick actions: "New Day Plan", "Start Today's Mission", "Open Settings"
- Recent pages
- Search results inline
- Keyboard navigable (arrow keys + enter)

---

## 11. DATA FLOW ARCHITECTURE

### Frontend → Backend Communication

```
SvelteKit Page/Component
    ↓ calls
$lib/commands/*.ts  (typed invoke wrappers)
    ↓ invoke()
Tauri IPC Bridge
    ↓ deserializes
src-tauri/src/commands/*.rs  (Tauri command handlers)
    ↓ calls
src-tauri/src/services/*.rs  (business logic)
    ↓ queries
src-tauri/src/db/*.rs  (SQLx queries against SQLite)
    ↓ returns
Result<T, AppError>
    ↓ serialized
JSON over IPC
    ↓ deserialized
TypeScript typed response
    ↓ updates
Svelte 5 $state / $derived reactivity
    ↓ renders
DOM
```

### State Management Pattern

Use Svelte 5 runes for ALL state. No external state management library.

```typescript
// src/lib/stores/app.svelte.ts

class AppState {
  activeProgram = $state<Program | null>(null);
  sidebarCollapsed = $state(false);
  commandPaletteOpen = $state(false);
  toasts = $state<Toast[]>([]);
  
  activeProgramId = $derived(this.activeProgram?.id ?? null);
  
  addToast(toast: Omit<Toast, 'id'>) {
    const id = crypto.randomUUID();
    this.toasts = [...this.toasts, { ...toast, id }];
    if (toast.type !== 'error') {
      setTimeout(() => this.removeToast(id), 5000);
    }
  }
  
  removeToast(id: string) {
    this.toasts = this.toasts.filter(t => t.id !== id);
  }
}

export const appState = new AppState();
```

### Autosave Pattern

```typescript
// In attempt page component
let autosaveTimer: ReturnType<typeof setInterval>;

$effect(() => {
  autosaveTimer = setInterval(async () => {
    if (isDirty) {
      await autosaveAttempt(attemptId, {
        exerciseNotes,
        codeSnapshot,
        whatBroke,
        whyBroke,
        howFixed,
        refactorTomorrow,
        dailySummary,
      });
      lastSaved = new Date();
      isDirty = false;
    }
  }, 5000);

  return () => clearInterval(autosaveTimer);
});
```

---

## 12. RUST ERROR HANDLING

```rust
// src-tauri/src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Dependency not met: {0}")]
    DependencyNotMet(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Import error: {0}")]
    Import(String),
}

// Convert to String for Tauri IPC serialization
impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}
```

---

## 13. TAURI v2 APP INITIALIZATION

```rust
// src-tauri/src/lib.rs

use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use tauri::Manager;

pub struct AppState {
    pub db: sqlx::SqlitePool,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;
            std::fs::create_dir_all(app_data_dir.join("artifacts"))?;

            let db_path = app_data_dir.join("buildops40.db");
            let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to create database pool");

                sqlx::migrate!("./src/db/migrations")
                    .run(&pool)
                    .await
                    .expect("Failed to run migrations");

                pool
            });

            app.manage(AppState { db: pool });

            // Initialize tracing
            tracing_subscriber::fmt()
                .with_env_filter("buildops40=debug")
                .init();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Programs
            commands::programs::create_program,
            commands::programs::get_program,
            commands::programs::list_programs,
            commands::programs::update_program,
            commands::programs::delete_program,
            commands::programs::duplicate_program,
            commands::programs::get_program_stats,
            // Modules
            commands::modules::create_module,
            commands::modules::get_module,
            commands::modules::list_modules,
            commands::modules::update_module,
            commands::modules::delete_module,
            commands::modules::reorder_modules,
            // Day Plans
            commands::days::create_day_plan,
            commands::days::get_day_plan,
            commands::days::list_day_plans,
            commands::days::list_day_plans_by_module,
            commands::days::update_day_plan,
            commands::days::delete_day_plan,
            commands::days::reorder_day_plans,
            commands::days::duplicate_day_plan,
            commands::days::add_checklist_item,
            commands::days::update_checklist_item,
            commands::days::delete_checklist_item,
            commands::days::reorder_checklist_items,
            commands::days::add_quiz_question,
            commands::days::update_quiz_question,
            commands::days::delete_quiz_question,
            commands::days::add_tag_to_day,
            commands::days::remove_tag_from_day,
            commands::days::create_concept_tag,
            commands::days::list_concept_tags,
            commands::days::add_dependency,
            commands::days::remove_dependency,
            commands::days::check_dependencies,
            // Attempts
            commands::attempts::start_attempt,
            commands::attempts::get_attempt,
            commands::attempts::get_current_attempt,
            commands::attempts::list_attempts,
            commands::attempts::autosave_attempt,
            commands::attempts::submit_attempt,
            commands::attempts::toggle_checklist_item,
            commands::attempts::start_memory_rebuild,
            commands::attempts::complete_memory_rebuild,
            commands::attempts::add_bug_log,
            commands::attempts::update_bug_log,
            commands::attempts::delete_bug_log,
            commands::attempts::list_bug_logs,
            commands::attempts::add_exercise_entry,
            commands::attempts::update_exercise_entry,
            commands::attempts::delete_exercise_entry,
            commands::attempts::reorder_exercise_entries,
            commands::attempts::submit_quiz_answer,
            commands::attempts::get_quiz_results,
            // Evidence
            commands::evidence::upload_artifact,
            commands::evidence::add_link_artifact,
            commands::evidence::add_code_artifact,
            commands::evidence::add_note_artifact,
            commands::evidence::delete_artifact,
            commands::evidence::list_artifacts,
            commands::evidence::open_artifact_file,
            // Analytics
            commands::analytics::get_dashboard_data,
            commands::analytics::get_skill_radar,
            commands::analytics::get_weekly_review,
            commands::analytics::generate_weekly_review,
            commands::analytics::get_burndown_data,
            commands::analytics::get_concept_heatmap,
            commands::analytics::get_score_history,
            commands::analytics::get_time_analytics,
            commands::analytics::get_bug_analytics,
            commands::analytics::get_streak_data,
            commands::analytics::use_streak_freeze,
            commands::analytics::get_badges,
            commands::analytics::get_forgetting_curve_alerts,
            // Spaced Repetition
            commands::spaced_repetition::get_due_reviews,
            commands::spaced_repetition::record_review,
            commands::spaced_repetition::get_review_schedule,
            // Search
            commands::search::search,
            commands::search::rebuild_search_index,
            // Export
            commands::export::export_program_pdf,
            commands::export::export_weekly_report_pdf,
            commands::export::export_program_json,
            commands::export::export_program_csv,
            commands::export::import_program_json,
            commands::export::export_day_plan_template,
            // Files & Settings
            commands::files::pick_file,
            commands::files::pick_save_location,
            commands::files::get_app_data_dir,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::attempts::start_time_log,
            commands::attempts::stop_time_log,
            commands::attempts::get_active_time_log,
        ])
        .run(tauri::generate_context!())
        .expect("Error running BuildOps 40");
}
```

```rust
// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
    buildops40::run();
}
```

---

## 14. TAURI v2 CAPABILITIES

```json
// src-tauri/capabilities/default.json
{
  "identifier": "default",
  "description": "Default capability for BuildOps 40",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "shell:allow-open",
    "dialog:allow-open",
    "dialog:allow-save",
    "dialog:allow-message",
    "dialog:allow-ask",
    "fs:allow-read",
    "fs:allow-write",
    "fs:allow-exists",
    "fs:allow-mkdir",
    "fs:allow-remove",
    "fs:allow-rename",
    "fs:allow-copy-file",
    "fs:default",
    "notification:default",
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
    "notification:allow-notify",
    "os:default",
    "clipboard-manager:allow-write",
    "clipboard-manager:allow-read"
  ]
}
```

---

## 15. BUILD AND DEVELOPMENT

### Prerequisites

```bash
# macOS development
xcode-select --install                    # Xcode command line tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust
cargo install sqlx-cli --features sqlite  # SQLx CLI for migrations
corepack enable && corepack prepare pnpm@latest --activate        # pnpm
```

### Development

```bash
# Install frontend dependencies
pnpm install

# Run migrations (creates DB if needed)
cd src-tauri && DATABASE_URL="sqlite:buildops40.db?mode=rwc" sqlx migrate run --source src/db/migrations

# Start development (launches both Vite + Tauri)
pnpm tauri dev
```

### Production Build

```bash
# Build for macOS (creates .app and .dmg)
pnpm tauri build

# Output: src-tauri/target/release/bundle/dmg/BuildOps 40_0.1.0_aarch64.dmg
# Output: src-tauri/target/release/bundle/macos/BuildOps 40.app
```

### Distribution

Direct `.dmg` distribution — no App Store. User opens DMG, drags to Applications. On first launch, macOS Gatekeeper may require right-click → Open. For future: code-sign with Developer ID for seamless opening.

---

## 16. IMPLEMENTATION ORDER

Build in this exact order. Each phase must compile and run before moving to the next.

### Phase 1 — Foundation (Build first)
1. Tauri v2 project scaffold with SvelteKit 5 frontend
2. SQLite database setup with all 19 migrations
3. Rust models for all entities
4. Program + Module CRUD commands
5. Root layout with sidebar navigation
6. Programs list page
7. Create program page

### Phase 2 — Core Learning Loop
8. Day Plan CRUD commands (including checklist, quiz questions)
9. Day Plan editor page (full authoring interface)
10. Day Mission view page
11. Day Attempt commands (start, autosave, submit)
12. Active Attempt working screen with CodeMirror editor
13. Checklist interaction
14. Score card submission with quality gate logic
15. Attempt history page

### Phase 3 — Evidence & Knowledge
16. Artifact commands (upload, link, code, note)
17. Evidence locker UI with file drop zone
18. Bug log CRUD
19. Exercise entry CRUD with CodeMirror
20. Markdown editor + preview components

### Phase 4 — Intelligence
21. Concept tag system
22. Dependency graph + validation
23. Spaced repetition engine (SM-2)
24. Streak tracking + freeze mechanic
25. Badge evaluation engine

### Phase 5 — Analytics
26. Dashboard with all widgets
27. Skill radar chart
28. Score trend line
29. Concept heatmap
30. Burndown chart
31. Time analytics
32. Bug analytics
33. Weekly review auto-generation
34. Forgetting curve alerts

### Phase 6 — Search & Export
35. FTS5 search with triggers
36. Global search UI + command palette
37. PDF export (program report + weekly report)
38. JSON export/import
39. CSV export

### Phase 7 — Polish
40. Settings page with all options
41. Theme switching (dark/light)
42. Keyboard shortcuts
43. Toast notification system
44. Loading skeletons for all pages
45. Empty states for all pages
46. Error boundaries for all pages
47. Window management (remember size/position)

---

## 17. TESTING EXPECTATIONS

- Every Rust command has at least one unit test
- Every service has integration tests against a test SQLite database
- Frontend: type checking passes with zero errors (`pnpm check`)
- Full `cargo clippy` pass with zero warnings
- The app compiles and launches on macOS 13+
- All 19 migrations run cleanly on a fresh database
- Create → Read → Update → Delete works for every entity

---

## 18. WHAT "DONE" LOOKS LIKE

The application is complete when:

1. A user can create a program, add modules, add day plans with full content
2. A user can start an attempt, write code in the editor, take notes, complete checklist items
3. A user can submit a quiz, get scored, and see quality gate results
4. A user can upload evidence files, add links, add code snippets
5. A user can log bugs with full symptom/cause/fix/prevention tracking
6. A user can do a memory rebuild with timer enforcement
7. Scores trigger correct status (blocked/passed/mastery)
8. Streaks track correctly with freeze support
9. Badges are awarded when conditions are met
10. Spaced repetition schedules reviews at correct intervals
11. Dependencies block advancement when prerequisites aren't met
12. Dashboard shows accurate real-time data
13. All analytics charts render with real data
14. Weekly reviews auto-generate from attempt data
15. Full-text search returns relevant results across all content
16. Export generates valid PDF, JSON, and CSV files
17. Import reconstructs a program from JSON export
18. Settings persist and take effect immediately
19. Autosave works reliably every 5 seconds during attempts
20. The app launches, runs, and closes cleanly on macOS

---

**END OF SPECIFICATION**

**Build it all. Build it right. Build it once.**
