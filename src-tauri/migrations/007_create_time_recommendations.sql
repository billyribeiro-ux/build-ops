CREATE TABLE IF NOT EXISTS time_recommendations (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL DEFAULT 'default',
    recommendation_type TEXT NOT NULL CHECK (recommendation_type IN ('increase_build', 'decrease_deep', 'add_buffer', 'adjust_break')),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    data_source TEXT NOT NULL,
    confidence_score REAL NOT NULL DEFAULT 0.0 CHECK (confidence_score >= 0.0 AND confidence_score <= 1.0),
    is_applied INTEGER NOT NULL DEFAULT 0,
    is_dismissed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    applied_at TEXT,
    dismissed_at TEXT
);

CREATE INDEX idx_time_recommendations_user ON time_recommendations(user_id);
CREATE INDEX idx_time_recommendations_type ON time_recommendations(recommendation_type);
CREATE INDEX idx_time_recommendations_active ON time_recommendations(is_applied, is_dismissed);
