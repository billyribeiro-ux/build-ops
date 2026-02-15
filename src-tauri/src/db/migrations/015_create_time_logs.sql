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
