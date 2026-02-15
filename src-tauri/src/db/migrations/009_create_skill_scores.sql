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
