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

    min_minutes INTEGER NOT NULL DEFAULT 90,
    recommended_minutes INTEGER NOT NULL DEFAULT 120,
    deep_minutes INTEGER NOT NULL DEFAULT 180,
    complexity_level INTEGER NOT NULL DEFAULT 3 CHECK (complexity_level >= 1 AND complexity_level <= 5),
    focus_blocks TEXT NOT NULL DEFAULT '[]',

    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(program_id, day_number, version)
);

CREATE INDEX idx_day_plans_program ON day_plans(program_id);
CREATE INDEX idx_day_plans_module ON day_plans(module_id);
CREATE INDEX idx_day_plans_day_number ON day_plans(program_id, day_number);
CREATE INDEX idx_day_plans_complexity ON day_plans(complexity_level);
