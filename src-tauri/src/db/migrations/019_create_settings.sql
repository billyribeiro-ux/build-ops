CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

INSERT OR IGNORE INTO settings (key, value) VALUES
    ('theme', '"dark"'),
    ('autosave_interval_ms', '5000'),
    ('default_session_minutes', '60'),
    ('memory_rebuild_minutes', '15'),
    ('blocked_threshold', '70'),
    ('mastery_threshold', '95'),
    ('streak_freezes_per_month', '2'),
    ('daily_reminder_enabled', 'true'),
    ('daily_reminder_time', '"09:00"'),
    ('spaced_repetition_enabled', 'true'),
    ('font_size', '14'),
    ('editor_theme', '"one-dark"'),
    ('sidebar_collapsed', 'false');
