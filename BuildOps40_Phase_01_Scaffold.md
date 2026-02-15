# BuildOps 40 — Phase 1: Project Scaffold, Database & Rust Foundation

## What This Phase Builds

This phase creates the entire project skeleton from nothing. By the end, you have a Tauri v2 macOS app that launches, connects to a SQLite database, runs all 19 migrations, and shows a blank SvelteKit shell with sidebar navigation. No features yet — just the foundation that every future phase builds on.

---

## Prerequisites

Before starting, the development machine must have:

```bash
# Xcode command line tools
xcode-select --install

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup target add aarch64-apple-darwin

# SQLx CLI
cargo install sqlx-cli --features sqlite

# Node.js (v20+) and pnpm
corepack enable && corepack prepare pnpm@latest --activate
```

---

## Step 1: Create the Tauri v2 + SvelteKit Project

Run the Tauri scaffold command and select SvelteKit as the frontend framework:

```bash
pnpm create tauri-app buildops40 --template sveltekit-ts
cd buildops40
pnpm install
```

If the scaffold doesn't produce a clean SvelteKit 5 + Tauri v2 setup, create it manually. The point is: after this step, `pnpm tauri dev` must launch a Tauri window showing a SvelteKit page.

---

## Step 2: Configure SvelteKit for Tauri (Static Adapter)

Tauri needs a static build, not a server-rendered one. Replace whatever adapter the scaffold installed.

**Install the static adapter:**

```bash
pnpm add -D @sveltejs/adapter-static
```

**File: `svelte.config.js`**

```javascript
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

**File: `vite.config.ts`**

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
});
```

---

## Step 3: Configure TypeScript Strict Mode

**File: `tsconfig.json`**

```json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": false,
    "moduleResolution": "bundler",
    "target": "ESNext",
    "module": "ESNext",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "sourceMap": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "allowJs": true,
    "checkJs": true
  }
}
```

Note: `exactOptionalPropertyTypes` is set to `false` because some SvelteKit internals don't comply with it. Everything else is strict.

---

## Step 4: Install All Frontend Dependencies

Run this single command to install everything the project needs across all phases. Install it all now so future phases don't need to stop for dependency installation.

```bash
pnpm add @tauri-apps/api @tauri-apps/plugin-dialog @tauri-apps/plugin-fs @tauri-apps/plugin-notification @tauri-apps/plugin-os @tauri-apps/plugin-shell @tauri-apps/plugin-clipboard-manager @iconify/svelte codemirror @codemirror/lang-javascript @codemirror/lang-html @codemirror/lang-css @codemirror/lang-json @codemirror/lang-markdown @codemirror/lang-rust @codemirror/lang-python @codemirror/theme-one-dark @codemirror/autocomplete @codemirror/lint echarts marked dompurify date-fns fuse.js bits-ui clsx tailwind-merge

pnpm add -D @sveltejs/adapter-static @tailwindcss/typography @tailwindcss/forms autoprefixer postcss tailwindcss prettier prettier-plugin-svelte svelte-check
```

---

## Step 5: Configure Tailwind CSS

**File: `tailwind.config.ts`**

```typescript
import type { Config } from 'tailwindcss';
import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        bg: {
          primary: 'var(--bg-primary)',
          secondary: 'var(--bg-secondary)',
          tertiary: 'var(--bg-tertiary)',
          hover: 'var(--bg-hover)',
        },
        border: {
          primary: 'var(--border-primary)',
          secondary: 'var(--border-secondary)',
        },
        text: {
          primary: 'var(--text-primary)',
          secondary: 'var(--text-secondary)',
          tertiary: 'var(--text-tertiary)',
        },
        accent: {
          primary: 'var(--accent-primary)',
          success: 'var(--accent-success)',
          warning: 'var(--accent-warning)',
          danger: 'var(--accent-danger)',
          info: 'var(--accent-info)',
          mastery: 'var(--accent-mastery)',
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'SF Mono', 'Fira Code', 'monospace'],
      },
    },
  },
  plugins: [forms, typography],
} satisfies Config;
```

**File: `postcss.config.js`**

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
```

**File: `src/app.css`**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

:root {
  /* Dark mode (default) */
  --bg-primary: #0A0A0B;
  --bg-secondary: #111113;
  --bg-tertiary: #1A1A1E;
  --bg-hover: #222228;
  --border-primary: #2A2A30;
  --border-secondary: #3A3A42;
  --text-primary: #FAFAFA;
  --text-secondary: #A1A1AA;
  --text-tertiary: #71717A;
  --accent-primary: #6366F1;
  --accent-success: #22C55E;
  --accent-warning: #F59E0B;
  --accent-danger: #EF4444;
  --accent-info: #3B82F6;
  --accent-mastery: #A855F7;
}

:root.light {
  --bg-primary: #FFFFFF;
  --bg-secondary: #F9FAFB;
  --bg-tertiary: #F3F4F6;
  --bg-hover: #E5E7EB;
  --border-primary: #E5E7EB;
  --border-secondary: #D1D5DB;
  --text-primary: #111827;
  --text-secondary: #6B7280;
  --text-tertiary: #9CA3AF;
}

@font-face {
  font-family: 'Inter';
  font-style: normal;
  font-weight: 100 900;
  font-display: swap;
  src: url('https://fonts.gstatic.com/s/inter/v13/UcCO3FwrK3iLTeHuS_fvQtMwCp50KnMw2boKoduKmMEVuLyfAZ9hiJ-Ek-_EeA.woff2') format('woff2');
}

html, body {
  height: 100%;
  margin: 0;
  padding: 0;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  font-family: 'Inter', system-ui, -apple-system, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Scrollbar styling */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: var(--bg-primary);
}

::-webkit-scrollbar-thumb {
  background: var(--border-secondary);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--text-tertiary);
}

/* Tauri window drag region */
.titlebar-drag-region {
  -webkit-app-region: drag;
}

.titlebar-no-drag {
  -webkit-app-region: no-drag;
}
```

---

## Step 6: Configure Tauri v2 Backend

**File: `src-tauri/Cargo.toml`**

Replace the entire contents with:

```toml
[package]
name = "buildops40"
version = "0.1.0"
description = "BuildOps 40 - Engineering Learning Operating System"
authors = ["Billy Ribeiro"]
edition = "2021"
rust-version = "1.77"

[lib]
name = "buildops40_lib"
crate-type = ["lib", "cdylib", "staticlib"]

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
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[profile.release]
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

Note: Dependencies like `argon2`, `genpdf`, `tantivy`, `reqwest`, `pdf-extract`, etc. are intentionally left out. They get added in the specific phase that needs them. This prevents compile time bloat during early development.

**File: `src-tauri/tauri.conf.json`**

Replace entirely:

```json
{
  "$schema": "https://raw.githubusercontent.com/nicklasmoeller/tauri-config-schema/main/schema.json",
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
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; font-src 'self' https://fonts.gstatic.com; script-src 'self'; img-src 'self' asset: https://asset.localhost"
    }
  },
  "bundle": {
    "active": true,
    "targets": ["dmg", "app"],
    "macOS": {
      "minimumSystemVersion": "13.0"
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

**File: `src-tauri/capabilities/default.json`**

```json
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

## Step 7: Create All 19 Database Migrations

Create the migrations directory at `src-tauri/src/db/migrations/`. Every migration file must be created exactly as shown. SQLx runs them in alphabetical order by filename.

**File: `src-tauri/src/db/migrations/001_create_programs.sql`**

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

**File: `src-tauri/src/db/migrations/002_create_modules.sql`**

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

**File: `src-tauri/src/db/migrations/003_create_day_plans.sql`**

```sql
CREATE TABLE IF NOT EXISTS day_plans (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    module_id TEXT NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    day_number INTEGER NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'archived')),
    syntax_targets TEXT NOT NULL DEFAULT '',
    implementation_brief TEXT NOT NULL DEFAULT '',
    files_to_create TEXT NOT NULL DEFAULT '',
    success_criteria TEXT NOT NULL DEFAULT '',
    stretch_challenge TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
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

**File: `src-tauri/src/db/migrations/004_create_day_attempts.sql`**

```sql
CREATE TABLE IF NOT EXISTS day_attempts (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    day_plan_version INTEGER NOT NULL DEFAULT 1,
    attempt_number INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'in_progress' CHECK (status IN ('in_progress', 'submitted', 'blocked', 'passed', 'mastery')),
    score_implementation INTEGER NOT NULL DEFAULT 0 CHECK (score_implementation >= 0 AND score_implementation <= 40),
    score_code_quality INTEGER NOT NULL DEFAULT 0 CHECK (score_code_quality >= 0 AND score_code_quality <= 20),
    score_accessibility INTEGER NOT NULL DEFAULT 0 CHECK (score_accessibility >= 0 AND score_accessibility <= 15),
    score_performance INTEGER NOT NULL DEFAULT 0 CHECK (score_performance >= 0 AND score_performance <= 15),
    score_quiz INTEGER NOT NULL DEFAULT 0 CHECK (score_quiz >= 0 AND score_quiz <= 10),
    total_score INTEGER GENERATED ALWAYS AS (
        score_implementation + score_code_quality + score_accessibility + score_performance + score_quiz
    ) STORED,
    memory_rebuild_completed INTEGER NOT NULL DEFAULT 0,
    memory_rebuild_passed INTEGER NOT NULL DEFAULT 0,
    memory_rebuild_notes TEXT NOT NULL DEFAULT '',
    what_broke TEXT NOT NULL DEFAULT '',
    why_broke TEXT NOT NULL DEFAULT '',
    how_fixed TEXT NOT NULL DEFAULT '',
    refactor_tomorrow TEXT NOT NULL DEFAULT '',
    daily_summary TEXT NOT NULL DEFAULT '',
    exercise_notes TEXT NOT NULL DEFAULT '',
    code_snapshot TEXT NOT NULL DEFAULT '',
    started_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    submitted_at TEXT,
    actual_minutes INTEGER NOT NULL DEFAULT 0,
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

**File: `src-tauri/src/db/migrations/005_create_checklists.sql`**

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

**File: `src-tauri/src/db/migrations/006_create_quizzes.sql`**

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

**File: `src-tauri/src/db/migrations/007_create_evidence.sql`**

```sql
CREATE TABLE IF NOT EXISTS artifacts (
    id TEXT PRIMARY KEY NOT NULL,
    attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    artifact_type TEXT NOT NULL CHECK (artifact_type IN ('file', 'screenshot', 'link', 'code_snippet', 'markdown_note')),
    title TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    file_path TEXT,
    file_name TEXT,
    file_size_bytes INTEGER,
    mime_type TEXT,
    url TEXT,
    code_content TEXT,
    code_language TEXT,
    markdown_content TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_artifacts_attempt ON artifacts(attempt_id);
CREATE INDEX idx_artifacts_type ON artifacts(artifact_type);
```

**File: `src-tauri/src/db/migrations/008_create_bug_logs.sql`**

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

**File: `src-tauri/src/db/migrations/009_create_skill_scores.sql`**

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

**File: `src-tauri/src/db/migrations/010_create_weekly_reviews.sql`**

```sql
CREATE TABLE IF NOT EXISTS weekly_reviews (
    id TEXT PRIMARY KEY NOT NULL,
    program_id TEXT NOT NULL REFERENCES programs(id) ON DELETE CASCADE,
    week_number INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    days_completed INTEGER NOT NULL DEFAULT 0,
    days_blocked INTEGER NOT NULL DEFAULT 0,
    average_score REAL NOT NULL DEFAULT 0.0,
    best_score INTEGER NOT NULL DEFAULT 0,
    worst_score INTEGER NOT NULL DEFAULT 0,
    total_time_minutes INTEGER NOT NULL DEFAULT 0,
    strong_concepts_json TEXT NOT NULL DEFAULT '[]',
    weak_concepts_json TEXT NOT NULL DEFAULT '[]',
    replay_recommendations_json TEXT NOT NULL DEFAULT '[]',
    summary TEXT NOT NULL DEFAULT '',
    goals_next_week TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, week_number)
);

CREATE INDEX idx_weekly_reviews_program ON weekly_reviews(program_id);
```

**File: `src-tauri/src/db/migrations/011_create_exercise_entries.sql`**

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

**File: `src-tauri/src/db/migrations/012_create_concept_tags.sql`**

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

**File: `src-tauri/src/db/migrations/013_create_spaced_repetition.sql`**

```sql
CREATE TABLE IF NOT EXISTS spaced_repetition (
    id TEXT PRIMARY KEY NOT NULL,
    day_plan_id TEXT NOT NULL REFERENCES day_plans(id) ON DELETE CASCADE,
    tag_id TEXT REFERENCES concept_tags(id) ON DELETE SET NULL,
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

**File: `src-tauri/src/db/migrations/014_create_dependencies.sql`**

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

**File: `src-tauri/src/db/migrations/015_create_time_logs.sql`**

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

**File: `src-tauri/src/db/migrations/016_create_streaks.sql`**

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

**File: `src-tauri/src/db/migrations/017_create_badges.sql`**

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

**File: `src-tauri/src/db/migrations/018_create_fts_index.sql`**

```sql
CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
    entity_id,
    entity_type,
    title,
    content,
    tags,
    tokenize='porter unicode61'
);
```

Note: FTS triggers are added in later phases when the entities they reference are built. Adding triggers now for entities that don't have commands yet would be premature.

**File: `src-tauri/src/db/migrations/019_create_settings.sql`**

```sql
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

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

## Step 8: Rust Source Files — Foundation

**File: `src-tauri/src/main.rs`**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

fn main() {
    buildops40_lib::run();
}
```

**File: `src-tauri/src/lib.rs`**

```rust
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

pub mod db;
pub mod error;

use sqlx::sqlite::SqlitePoolOptions;
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

            tracing_subscriber::fmt()
                .with_env_filter("buildops40=debug,sqlx=warn")
                .init();

            tracing::info!("Database path: {}", db_path.display());

            let pool = tauri::async_runtime::block_on(async {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("Failed to create database pool");

                // Enable WAL mode for better concurrent read performance
                sqlx::query("PRAGMA journal_mode=WAL")
                    .execute(&pool)
                    .await
                    .expect("Failed to enable WAL mode");

                // Enable foreign keys
                sqlx::query("PRAGMA foreign_keys=ON")
                    .execute(&pool)
                    .await
                    .expect("Failed to enable foreign keys");

                // Run migrations
                db::run_migrations(&pool).await;

                pool
            });

            app.manage(AppState { db: pool });

            tracing::info!("BuildOps 40 initialized successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // No commands yet — added in Phase 2+
        ])
        .run(tauri::generate_context!())
        .expect("Error running BuildOps 40");
}
```

**File: `src-tauri/src/error.rs`**

```rust
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

impl From<AppError> for String {
    fn from(err: AppError) -> Self {
        err.to_string()
    }
}
```

**File: `src-tauri/src/db/mod.rs`**

```rust
use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) {
    let migrations_dir = std::path::Path::new("src/db/migrations");

    if !migrations_dir.exists() {
        tracing::warn!("Migrations directory not found at {:?}, skipping", migrations_dir);
        return;
    }

    let mut entries: Vec<_> = std::fs::read_dir(migrations_dir)
        .expect("Failed to read migrations directory")
        .filter_map(Result::ok)
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "sql")
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let sql = std::fs::read_to_string(entry.path())
            .unwrap_or_else(|_| panic!("Failed to read migration: {:?}", entry.path()));

        tracing::info!("Running migration: {:?}", entry.file_name());

        // Split by semicolons and execute each statement
        // (SQLx execute doesn't support multi-statement by default for some drivers)
        for statement in sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(pool)
                    .await
                    .unwrap_or_else(|e| {
                        panic!(
                            "Migration failed: {:?} — Error: {} — Statement: {}",
                            entry.file_name(),
                            e,
                            &trimmed[..trimmed.len().min(100)]
                        );
                    });
            }
        }
    }

    tracing::info!("All migrations completed successfully");
}
```

**File: `src-tauri/build.rs`**

```rust
fn main() {
    tauri_build::build();
}
```

---

## Step 9: Frontend Shell — Root Layout with Sidebar

**File: `src/app.html`**

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <link rel="icon" href="%sveltekit.assets%/favicon.png" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    %sveltekit.head%
  </head>
  <body data-sveltekit-prerender="false">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

**File: `src/app.d.ts`**

```typescript
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}

export {};
```

**File: `src/lib/utils/cn.ts`**

```typescript
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}
```

**File: `src/lib/config/navigation.ts`**

```typescript
export interface NavItem {
  label: string;
  href: string;
  icon: string;
  shortcut?: string;
}

export const mainNavItems: NavItem[] = [
  { label: 'Dashboard', href: '/', icon: 'ph:house-bold', shortcut: '⌘1' },
  { label: 'Programs', href: '/programs', icon: 'ph:books-bold', shortcut: '⌘2' },
  { label: 'Analytics', href: '/analytics', icon: 'ph:chart-line-bold', shortcut: '⌘3' },
  { label: 'Search', href: '/search', icon: 'ph:magnifying-glass-bold', shortcut: '⌘4' },
  { label: 'Export', href: '/export', icon: 'ph:export-bold', shortcut: '⌘5' },
];

export const bottomNavItems: NavItem[] = [
  { label: 'Import', href: '/import', icon: 'ph:upload-bold' },
  { label: 'Settings', href: '/settings', icon: 'ph:gear-bold' },
];
```

**File: `src/lib/components/layout/Sidebar.svelte`**

```svelte
<script lang="ts">
  import Icon from '@iconify/svelte';
  import { page } from '$app/stores';
  import { mainNavItems, bottomNavItems } from '$lib/config/navigation';

  let { collapsed = false }: { collapsed?: boolean } = $props();

  let currentPath = $derived($page.url.pathname);

  function isActive(href: string): boolean {
    if (href === '/') return currentPath === '/';
    return currentPath.startsWith(href);
  }
</script>

<aside
  class="fixed left-0 top-0 bottom-0 z-40 flex flex-col border-r border-border-primary bg-bg-secondary transition-all duration-200"
  style="width: {collapsed ? '48px' : '240px'}; padding-top: 48px;"
>
  <!-- Logo -->
  <div class="flex items-center gap-3 px-4 py-4">
    {#if !collapsed}
      <span class="text-lg font-bold text-text-primary">BuildOps 40</span>
    {/if}
  </div>

  <!-- Main nav -->
  <nav class="flex flex-1 flex-col gap-1 px-2">
    {#each mainNavItems as item}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {isActive(item.href) ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <Icon icon={item.icon} width="20" />
        {#if !collapsed}
          <span>{item.label}</span>
          {#if item.shortcut}
            <span class="ml-auto text-xs text-text-tertiary">{item.shortcut}</span>
          {/if}
        {/if}
      </a>
    {/each}
  </nav>

  <!-- Bottom nav -->
  <nav class="flex flex-col gap-1 px-2 pb-4">
    {#each bottomNavItems as item}
      <a
        href={item.href}
        class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors {isActive(item.href) ? 'bg-accent-primary/10 text-accent-primary' : 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
      >
        <Icon icon={item.icon} width="20" />
        {#if !collapsed}
          <span>{item.label}</span>
        {/if}
      </a>
    {/each}
  </nav>
</aside>
```

**File: `src/lib/components/layout/TopBar.svelte`**

```svelte
<script lang="ts">
  import Icon from '@iconify/svelte';

  let { sidebarWidth = 240 }: { sidebarWidth?: number } = $props();
</script>

<header
  class="titlebar-drag-region fixed top-0 right-0 z-50 flex h-12 items-center border-b border-border-primary bg-bg-secondary/80 backdrop-blur-md"
  style="left: {sidebarWidth}px;"
>
  <!-- macOS traffic lights occupy left ~70px, this bar starts after sidebar -->
  <div class="titlebar-no-drag flex flex-1 items-center gap-4 px-4">
    <!-- Search trigger -->
    <button
      class="flex items-center gap-2 rounded-lg border border-border-primary bg-bg-tertiary px-3 py-1.5 text-sm text-text-tertiary transition-colors hover:border-border-secondary hover:text-text-secondary"
      onclick={() => {/* Command palette - implemented in Phase 10 */}}
    >
      <Icon icon="ph:magnifying-glass" width="16" />
      <span>Search...</span>
      <kbd class="ml-4 rounded bg-bg-hover px-1.5 py-0.5 text-xs">⌘K</kbd>
    </button>
  </div>
</header>
```

**File: `src/routes/+layout.svelte`**

```svelte
<script lang="ts">
  import '../app.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import TopBar from '$lib/components/layout/TopBar.svelte';

  let { children } = $props();

  let sidebarCollapsed = $state(false);
  let sidebarWidth = $derived(sidebarCollapsed ? 48 : 240);
</script>

<div class="flex h-screen bg-bg-primary">
  <Sidebar collapsed={sidebarCollapsed} />

  <div class="flex flex-1 flex-col" style="margin-left: {sidebarWidth}px;">
    <TopBar {sidebarWidth} />

    <main class="flex-1 overflow-y-auto pt-12">
      <div class="mx-auto max-w-7xl p-6">
        {@render children()}
      </div>
    </main>
  </div>
</div>
```

**File: `src/routes/+page.svelte`**

```svelte
<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<div class="space-y-6">
  <div class="flex items-center gap-3">
    <Icon icon="ph:house-bold" width="28" class="text-accent-primary" />
    <h1 class="text-2xl font-bold text-text-primary">Dashboard</h1>
  </div>

  <div class="rounded-xl border border-border-primary bg-bg-secondary p-8 text-center">
    <Icon icon="ph:rocket-launch-bold" width="48" class="mx-auto mb-4 text-accent-primary" />
    <h2 class="mb-2 text-xl font-semibold text-text-primary">Welcome to BuildOps 40</h2>
    <p class="text-text-secondary">
      Your engineering learning OS is ready. Create your first program to get started.
    </p>
    <a
      href="/programs/new"
      class="mt-6 inline-flex items-center gap-2 rounded-lg bg-accent-primary px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-accent-primary/90"
    >
      <Icon icon="ph:plus-bold" width="16" />
      Create Program
    </a>
  </div>
</div>
```

Create placeholder pages for every top-level route so SvelteKit doesn't 404:

**File: `src/routes/programs/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Programs</h1>
<p class="mt-2 text-text-secondary">Program management — built in Phase 2.</p>
```

**File: `src/routes/analytics/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Analytics</h1>
<p class="mt-2 text-text-secondary">Analytics dashboard — built in Phase 9.</p>
```

**File: `src/routes/search/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Search</h1>
<p class="mt-2 text-text-secondary">Global search — built in Phase 10.</p>
```

**File: `src/routes/export/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Export</h1>
<p class="mt-2 text-text-secondary">Export center — built in Phase 11.</p>
```

**File: `src/routes/import/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Import</h1>
<p class="mt-2 text-text-secondary">PDF import pipeline — built in Phase 12.</p>
```

**File: `src/routes/settings/+page.svelte`**

```svelte
<h1 class="text-2xl font-bold text-text-primary">Settings</h1>
<p class="mt-2 text-text-secondary">App settings — built in Phase 13.</p>
```

---

## Step 10: Verification

After completing all steps above, run the following checks. Every single one must pass.

### Check 1: Frontend compiles

```bash
pnpm check
```

Expected: Zero errors, zero warnings.

### Check 2: Rust compiles

```bash
cd src-tauri && cargo build
```

Expected: Successful compilation. Warnings from clippy about unused code are acceptable in this phase only (since we haven't built commands yet).

### Check 3: App launches

```bash
pnpm tauri dev
```

Expected: A macOS window opens with the title "BuildOps 40". You see a dark-themed app with a sidebar on the left showing navigation items (Dashboard, Programs, Analytics, Search, Export, Import, Settings). The main content area shows the welcome screen with "Welcome to BuildOps 40" and a "Create Program" button. Clicking sidebar links navigates to placeholder pages.

### Check 4: Database created

After launching the app once, check that the database was created:

```bash
ls ~/Library/Application\ Support/com.billyribeiro.buildops40/
```

Expected: You see `buildops40.db` and an `artifacts/` directory.

### Check 5: All tables exist

```bash
sqlite3 ~/Library/Application\ Support/com.billyribeiro.buildops40/buildops40.db ".tables"
```

Expected: You see all tables: `programs`, `modules`, `day_plans`, `day_attempts`, `checklist_items`, `attempt_checklist`, `quiz_questions`, `quiz_attempts`, `artifacts`, `bug_logs`, `skill_scores`, `weekly_reviews`, `exercise_entries`, `concept_tags`, `day_plan_tags`, `spaced_repetition`, `day_dependencies`, `time_logs`, `streaks`, `badges`, `search_index`, `settings`.

### Check 6: Default settings populated

```bash
sqlite3 ~/Library/Application\ Support/com.billyribeiro.buildops40/buildops40.db "SELECT * FROM settings;"
```

Expected: All 13 default settings rows present.

---

## What's Done After This Phase

- Tauri v2 macOS app scaffold with correct bundle config
- SvelteKit 5 frontend with static adapter
- TypeScript strict mode configured
- Tailwind CSS with full design token system (dark + light mode CSS variables)
- All 19 database migration files created and running
- Rust error types defined
- Database connection pool with WAL mode and foreign keys enabled
- Root layout with sidebar navigation and top bar
- Placeholder pages for all top-level routes
- App launches, displays UI, creates database, runs migrations

## What's NOT Done (Deferred to Later Phases)

- No Tauri commands registered (Phase 2+)
- No Rust models or services (Phase 2+)
- No data fetching or display (Phase 2+)
- No CodeMirror editor integration (Phase 4)
- No charts or analytics (Phase 9)
- No search functionality (Phase 10)
- No export/import (Phase 11-12)
- FTS triggers not yet attached (Phase 10)

---

## Next Phase

Phase 2 builds Program & Module CRUD — the first real data flow through the entire stack from UI to database and back.
