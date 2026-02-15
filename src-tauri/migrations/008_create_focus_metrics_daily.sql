CREATE TABLE IF NOT EXISTS focus_metrics_daily (
    id TEXT PRIMARY KEY NOT NULL,
    date TEXT NOT NULL,
    user_id TEXT NOT NULL DEFAULT 'default',
    total_planned_minutes INTEGER NOT NULL DEFAULT 0,
    total_actual_minutes INTEGER NOT NULL DEFAULT 0,
    variance_percentage REAL NOT NULL DEFAULT 0.0,
    completion_rate REAL NOT NULL DEFAULT 0.0,
    focus_efficiency REAL NOT NULL DEFAULT 0.0,
    deep_work_minutes INTEGER NOT NULL DEFAULT 0,
    interruption_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(date, user_id)
);

CREATE INDEX idx_focus_metrics_date ON focus_metrics_daily(date);
CREATE INDEX idx_focus_metrics_user ON focus_metrics_daily(user_id);
