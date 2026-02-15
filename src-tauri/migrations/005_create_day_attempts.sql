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
