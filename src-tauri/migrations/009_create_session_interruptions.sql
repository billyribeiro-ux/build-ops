CREATE TABLE IF NOT EXISTS session_interruptions (
    id TEXT PRIMARY KEY NOT NULL,
    session_id TEXT NOT NULL REFERENCES day_sessions(id) ON DELETE CASCADE,
    interruption_type TEXT NOT NULL CHECK (interruption_type IN ('external', 'mental', 'technical', 'break')),
    duration_seconds INTEGER NOT NULL DEFAULT 0,
    notes TEXT NOT NULL DEFAULT '',
    occurred_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

CREATE INDEX idx_interruptions_session ON session_interruptions(session_id);
CREATE INDEX idx_interruptions_type ON session_interruptions(interruption_type);
CREATE INDEX idx_interruptions_occurred ON session_interruptions(occurred_at);
