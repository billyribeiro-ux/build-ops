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
