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
