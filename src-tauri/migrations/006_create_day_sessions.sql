CREATE TABLE IF NOT EXISTS day_sessions (
    id TEXT PRIMARY KEY NOT NULL,
    day_attempt_id TEXT NOT NULL REFERENCES day_attempts(id) ON DELETE CASCADE,
    session_type TEXT NOT NULL CHECK (session_type IN ('learn', 'build', 'debug', 'rebuild', 'quiz', 'review')),
    planned_minutes INTEGER NOT NULL DEFAULT 0,
    actual_minutes INTEGER NOT NULL DEFAULT 0,
    started_at TEXT,
    ended_at TEXT,
    status TEXT NOT NULL DEFAULT 'planned' CHECK (status IN ('planned', 'in_progress', 'done', 'skipped')),
    notes TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_day_sessions_attempt ON day_sessions(day_attempt_id);
CREATE INDEX idx_day_sessions_type ON day_sessions(session_type);
CREATE INDEX idx_day_sessions_status ON day_sessions(status);
CREATE INDEX idx_day_sessions_started ON day_sessions(started_at);
