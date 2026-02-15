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
