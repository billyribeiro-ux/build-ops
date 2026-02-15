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
